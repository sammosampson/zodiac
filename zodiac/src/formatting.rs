use legion::*;
use legion::world::*;
use zodiac_entities::*;
use zodiac_rendering_glium::*;

pub trait Pretty {
    fn to_pretty(&mut self);
}

impl Pretty for World {
    fn to_pretty(&mut self) {
        let mut registry = Registry::<String>::default();
        registry.register::<AbstractSyntaxNodeType>("AbstractSyntaxNodeType".to_string());
        registry.register::<BuildError>("BuildError".to_string());
        registry.register::<BuildError>("BuildError".to_string());
        registry.register::<BuildErrorOccurrence>("BuildErrorOccurrence".to_string());
        registry.register::<SourceFile>("SourceFile".to_string());
        registry.register::<SourceFileParsed>("SourceFileParsed".to_string());
        registry.register::<SourceFileChange>("SourceFileChange".to_string());
        registry.register::<SourceFileCreation>("SourceFileCreation".to_string());
        registry.register::<SourceFileInitialRead>("SourceFileInitialRead".to_string());
        registry.register::<SourceFileRemoval>("SourceFileRemoval".to_string());
        registry.register::<SourceFileRoot>("SourceFileRoot".to_string());
        registry.register::<SourceImplementation>("SourceImplementation".to_string());
        registry.register::<Removed>("Removed".to_string());
        registry.register::<Relationship>("Relationship".to_string());
        registry.register::<Root>("Root".to_string());
        registry.register::<Control>("Control".to_string());
        registry.register::<Rebuild>("Rebuild".to_string());
        registry.register::<RootWindowResized>("RootWindowResized".to_string());
        registry.register::<CurrentLayoutConstraints>("CurrentLayoutConstraints".to_string());
        registry.register::<Resized>("Resized".to_string());
        registry.register::<Mapped>("Mapped".to_string());
        registry.register::<Import>("Import".to_string());
        registry.register::<LayoutType>("LayoutType".to_string());
        registry.register::<LayoutContent>("LayoutContent".to_string());
        registry.register::<LayoutRequest>("LayoutRequest".to_string());
        registry.register::<LayoutChange>("LayoutChange".to_string());
        registry.register::<RenderType>("RenderType".to_string());
        registry.register::<Renderable>("Renderable".to_string());
        registry.register::<Name>("Name".to_string());
        registry.register::<Path>("Path".to_string());
        registry.register::<Character>("Character".to_string());
        registry.register::<Left>("Left".to_string());
        registry.register::<Top>("Top".to_string());
        registry.register::<OffsetsMapped>("OffsetsMapped".to_string());
        registry.register::<Width>("Width".to_string());
        registry.register::<MinimumWidth>("MinimumWidth".to_string());
        registry.register::<Height>("Height".to_string());
        registry.register::<MinimumHeight>("MinimumHeight".to_string());
        registry.register::<Radius>("Radius".to_string());
        registry.register::<GlyphIndex>("GlyphIndex".to_string());
        registry.register::<Colour>("Colour".to_string());
        registry.register::<StrokeWidth>("StrokeWidth".to_string());
        registry.register::<StrokeColour>("StrokeColour".to_string());
        registry.register::<CornerRadii>("CornerRadii".to_string());
        registry.register::<RenderPrimitive>("RenderPrimitive".to_string());
        let json = serde_json::to_value(self.as_serializable(passthrough(), &registry)).unwrap();
        println!("{:#}", json);
    }
}

impl<'a> Pretty for SubWorld<'a> {
    fn to_pretty(&mut self) {
        todo!()
    }
}