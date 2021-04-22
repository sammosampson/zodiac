use log::{info};
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

    let location = source_location_lookup.get(entity).unwrap();
    let source_text = source_reader.read_source_at_location(location).unwrap();
        
    info!("Source is now {:?} chars", source_text.len());

    let tokenizer = AbstractSyntaxTokenizer
        ::from_source(SourceTokenizer::from_string(&source_text));
    
    let tokens: Vec::<AbstractSyntaxTokenResult> = tokenizer
        .into_iter()
        .collect();

    if contains_root(&tokens) {
        command_buffer.add_component(*entity, SourceFileRoot::default());
    }

    source_tokens_lookup.insert(*entity, tokens); 
    
    command_buffer.add_component(*entity, SourceFileParsed::default());
}