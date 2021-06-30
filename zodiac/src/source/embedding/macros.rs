
#[macro_export]
macro_rules! element_creator_func {
    (
        $name:ident
    ) => {
        paste::item! {
            pub fn $name() -> [<$name:camel Builder>] {
                [<$name:camel Builder>]::new()
            }
        }
    }
}

#[macro_export]
macro_rules! element {
    (
        <$name:ident>
        [$component:expr]
        $(extra_components {$(
            [$extra_component:expr]
        )*})?
        $(attributes {$(
            $attr:ident $(($attr_ty:ty))?
        )*})?
    ) => {
        paste::item! {
            element_creator_func!($name);
            
            #[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
            pub enum [<$name:camel Attribute>] {
                None,
                $($(
                [<$attr:camel>]($($attr_ty)?),
                )*)?
            }

            impl Default for [<$name:camel Attribute>] {
                fn default() -> Self {
                    [<$name:camel Attribute>]::None
                }
            }

            #[derive(Default, PartialEq, Clone)]
            pub struct [<$name:camel Builder>] {
                children: Vec<u64>,
                attributes: Vec<[<$name:camel Attribute>]>,
                style: Option<u64>
            }

            impl [<$name:camel Builder>] {
                pub fn new() -> Self {
                    Self {
                        children: vec!(),
                        attributes: vec!(),
                        style: None
                    }
                }

                #[illicit::from_env(state: &moxie::Key<SourceBuildChangeState>)]
                #[illicit::from_env(cache: &moxie::Key<NodeBuildCache>)]
                pub fn build(&self) -> Node {
                    moxie::cache(
                        self,
                        |_| {    
                            let node = Node::new();
                            state.push_change([<$name:camel Change>]::between(node.id, self, &self.previous_version(node.id)));
                            cache.cache_current_node_revision(node.id, self.clone());
                            node
                        }
                    )
                }
                
                #[illicit::from_env(cache: &moxie::Key<NodeBuildCache>)]
                fn previous_version(&self, node_id: u64) -> Self {
                    cache.get_previous_node_revision(node_id, [<$name:camel Builder>]::default())
                }

                pub fn child(mut self, mut child: Node) -> Self {
                    if child.is_group { 
                        self.children.append(&mut child.group_children)
                    } else {
                        self.children.push(child.id);
                    }
                    self
                }

                pub fn style(mut self, value: Node) -> Self {
                    self.style = Some(value.id);
                    self
                }

                $($(
                pub fn $attr(mut self, value: $($attr_ty)?) -> Self {
                    self.attributes.push([<$name:camel Attribute>]::[<$attr:camel>](value));
                    self
                }
                )*)?
            }

            #[derive(Default, Debug, Clone)]
            pub struct [<$name:camel Change>] {
                node_id: u64,
                style: Option<u64>,
                child_changes: NodeChanges::<u64>,
                attribute_changes: NodeChanges::<[<$name:camel Attribute>]>
            }

            impl [<$name:camel Change>] {
                fn between(node_id: u64, current: &[<$name:camel Builder>], previous: &[<$name:camel Builder>]) -> Self {
                    Self {
                        node_id,
                        style: current.style,
                        child_changes: NodeChanges::<u64>::between(&current.children, &previous.children),
                        attribute_changes: NodeChanges::<[<$name:camel Attribute>]>::between(
                            &current.attributes, 
                            &previous.attributes)
                    }
                }
            }

            impl SourceBuildChange for [<$name:camel Change>] {
                fn apply<'a>(&self, command_buffer: &mut legion::systems::CommandBuffer, maps: &mut SourceBuildMaps<'a>) {        
                    let parent = command_buffer.get_or_create(self.node_id, || $component, maps);
                    $($(
                    command_buffer.add_component(parent, $extra_component);
                    )*)?
                    
                    command_buffer.add_component(parent, Rebuild::default());

                    if let Some(style_id) = self.style {
                        if let Some(style_entity) = maps.entity_map.get(&style_id) {
                            command_buffer.add_component(*style_entity, StyleRelationship::from(parent));
                        }
                    }
                    
                    self.child_changes.process_additions(&mut |child_id| command_buffer.add_child(parent, child_id, maps));    
                    self.child_changes.process_removals(&mut |child_id| command_buffer.remove_child(child_id, maps));
                    
                    self.attribute_changes.process_additions(&mut |attribute| {
                        match attribute {
                            $($(
                            [<$name:camel Attribute>]::[<$attr:camel>](value) => 
                                command_buffer.add_component(parent, [<$attr:camel>]::from(value.clone())),
                            )*)?
                            [<$name:camel Attribute>]::None => {}
                        }
                    });    
            
                    self.attribute_changes.process_removals(&mut |attribute| {
                        match attribute {
                            $($(
                            [<$name:camel Attribute>]::[<$attr:camel>](_) => 
                                command_buffer.remove_component::<[<$attr:camel>]>(parent),
                            )*)?
                            [<$name:camel Attribute>]::None => {}
                        }
                    });
                }
            }
        }
    }
}