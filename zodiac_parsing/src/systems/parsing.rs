use legion::*;
use legion::systems::*;
use zodiac_entities::*;
use crate::tokenization::abstract_syntax::*;
use crate::tokenization::source::*;
use crate::source::*;

#[system(for_each)]
#[filter(component::<SourceFile>())]
#[filter(!component::<SourceFileParsed>())]
#[filter(component::<SourceFileRemoval>())]
pub fn source_token_removal (
    entity: &Entity,
    command_buffer: &mut CommandBuffer,
    #[resource] source_tokens_lookup: &mut SourceTokensLookup
) {
    source_tokens_lookup.remove(entity); 
    command_buffer.add_component(*entity, SourceFileParsed::default());
}

#[system(for_each)]
#[filter(component::<SourceFile>())]
#[filter(!component::<SourceFileParsed>())]
#[filter(!component::<SourceFileRemoval>())]
pub fn source_parse<T:SourceReader + 'static> (
    entity: &Entity,
    command_buffer: &mut CommandBuffer,
    #[resource] source_location_lookup: &mut SourceLocationLookup,
    #[resource] source_tokens_lookup: &mut SourceTokensLookup,
    #[resource] source_reader: &mut T
) {

    let location = source_location_lookup.get(entity).unwrap(); // TODO: error source does not exist
    let source_text = source_reader.read_source_at_location(location).unwrap();  // TODO: error source cannot be read
        
    println!("Source is now {:?} chars", source_text.len());

    let tokens: Result<Vec<AbstractSyntaxToken>, AbstractSyntaxTokenError> = AbstractSyntaxTokenizer
        ::from_source(SourceTokenizer::from_string(&source_text))
        .into_iter()
        .collect();

    let tokens = tokens.unwrap(); // TODO: error token error

    let is_root = tokens.iter().any(|token| *token == AbstractSyntaxToken::Root);
    
    source_tokens_lookup.insert(*entity, tokens); 
    
    command_buffer.add_component(*entity, SourceFileParsed::default());

    if is_root {
        command_buffer.add_component(*entity, SourceFileRoot::default());
    }
}