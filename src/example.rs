use patternfly_yew::prelude::*;
use yew::prelude::*;

/// Include an example from an external file.
///
/// ```rust
/// # use yew::prelude::*;
/// fn example() -> Html {
///     let example1 = example!{ "Example" => "file.example" };
///     html!{
///         <div>{example1}</div>
///     }
/// }
/// ```
#[macro_export]
macro_rules! example {
    ($title:expr => $file:expr) => {{

        html! {
            <>
                <$crate::example::Example title={$title} code={include_str!($file)}>{{include!($file)}}</$crate::example::Example>
            </>
        }
    }};
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
    #[prop_or_default]
    pub subtitle: Children,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ExamplePage)]
pub fn example_page(props: &Props) -> Html {
    html! (
        <>
            <PageSection
                variant={PageSectionVariant::Light}
                limit_width=true
                sticky={[PageSectionSticky::Top]}
            >
                <Content>
                    <Title size={Size::XXXXLarge}>
                        { &props.title }
                    </Title>
                    { for props.subtitle.iter() }
                </Content>
            </PageSection>
            { for props.children.iter().map(|child|{
                html!(<PageSection>{child}</PageSection>)
            })}
        </>
    )
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ExampleProps {
    pub title: String,
    pub children: Children,
    pub code: String,
}

#[function_component(Example)]
pub fn example(props: &ExampleProps) -> Html {
    html!(
        <Flex modifiers={[FlexModifier::Column]}>

            <FlexItem>
                <Title level={Level::H2} size={Size::XXLarge}>
                    { &props.title }
                </Title>
            </FlexItem>

            <Flex>

                <FlexItem modifiers={[FlexModifier::Flex1]}>
                    <Title level={Level::H3} size={Size::Large}>{"Example"}</Title>
                    <Panel>
                        <PanelMain>
                            { for props.children.iter() }
                        </PanelMain>
                    </Panel>
                </FlexItem>

                <FlexItem modifiers={[FlexModifier::Flex1]}>
                    <Title level={Level::H3} size={Size::Large}>{"Code"}</Title>

                    <div class="pf-c-code-editor">
                        <div class="pf-c-code-editor__main">
                            <div class="pf-c-code-editor__code">
                                <pre class="pf-c-code-editor__code-pre">
                                    {&props.code}
                                </pre>
                            </div>
                        </div>
                    </div>

                </FlexItem>
            </Flex>

        </Flex>
    )
}
