use std::fmt::{Display, Error, Formatter};
use std::marker::PhantomData;

// use stdweb::web::event::*;
use stdweb::web::{Element, EventListenerHandle, IEventTarget};

use yew::html;
use yew::html::{Component, Html};
use yew::virtual_dom::vnode::VNode;
use yew::virtual_dom::vtag::VTag;
use yew::virtual_dom::vtext::VText;
use yew::virtual_dom::Listener;

use crate::dom::VNode as DomVNode;
use crate::events::EventHandler;
use crate::OutputType;

/// DOM output using the stdweb crate
pub struct Yew<COMP: Component> {
    component_type: PhantomData<COMP>,
}

impl<COMP: Component> OutputType for Yew<COMP> {
    type Events = Events<COMP>;
    type EventTarget = Element;
    type EventListenerHandle = EventListenerHandle;
}

macro_rules! declare_events_yew {
    ($($name:ident : $type:ty ,)*) => {
        /// Container type for DOM events.
        pub struct Events<COMP: Component> {
            $(
                pub $name: Option<Box<dyn EventHandler<Yew<COMP>, $type>>>,
            )*
        }

        impl<COMP: Component> Default for Events<COMP> {
            fn default() -> Self {
                Events {
                    $(
                        $name: None,
                    )*
                }
            }
        }

        /// Iterate over the defined events on a DOM object.
        #[macro_export]
        macro_rules! for_events_yew {
            ($event:ident in $events:expr => $body:block) => {
                $(
                    if let Some(ref mut $event) = $events.$name $body
                )*
            }
        }
    }
}

// TODO? these are all the "on*" attributes defined in the HTML5 standard, with
// the ones I've been unable to match to stdweb event types commented out.
//
// This needs review.

declare_events_yew! {
    // abort: html::?::Event,
    // autocomplete: html::?::Event,
    // autocompleteerror: html::?::Event,
    blur: html::onblur::Event,
    // cancel: html::?::Event,
    // canplay: html::?::Event,
    // canplaythrough: html::?::Event,
    change: html::onchange::Event,
    click: html::onclick::Event,
    // close: html::?::Event,
    contextmenu: html::oncontextmenu::Event,
    // cuechange: html::?::Event,
    dblclick: html::ondoubleclick::Event,
    drag: html::ondrag::Event,
    dragend: html::ondragend::Event,
    dragenter: html::ondragenter::Event,
    dragexit: html::ondragexit::Event,
    dragleave: html::ondragleave::Event,
    dragover: html::ondragover::Event,
    dragstart: html::ondragstart::Event,
    drop: html::ondrop::Event,
    // durationchange: html::?::Event,
    // emptied: html::?::Event,
    // ended: html::?::Event,
    // error: html::?::Event,
    focus: html::onfocus::Event,
    // ?: html::ongotpointercapture::Event,
    input: html::oninput::Event,
    // invalid: html::?::Event,
    keydown: html::onkeydown::Event,
    keypress: html::onkeypress::Event,
    keyup: html::onkeyup::Event,
    // load: html::?::Event,
    // loadeddata: html::?::Event,
    // loadedmetadata: html::?::Event,
    // loadstart: html::?::Event,
    // ?: html::onlostpointercapture::Event,
    mousedown: html::onmousedown::Event,
    mouseenter: html::onmouseenter::Event,
    mouseleave: html::onmouseleave::Event,
    mousemove: html::onmousemove::Event,
    mouseout: html::onmouseout::Event,
    mouseover: html::onmouseover::Event,
    mouseup: html::onmouseup::Event,
    mousewheel: html::onmousewheel::Event,
    // pause: html::?::Event,
    // play: html::?::Event,
    // playing: html::?::Event,
    // ?: html::onpointercancel::Event,
    // ?: html::onpointerdown::Event,
    // ?: html::onpointerenter::Event,
    // ?: html::onpointerleave::Event,
    // ?: html::onpointermove::Event,
    // ?: html::onpointerout::Event,
    // ?: html::onpointerover::Event,
    // ?: html::onpointerup::Event,
    // progress: html::?::Event,
    // ratechange: html::?::Event,
    // reset: html::?::Event,
    // resize: html::?::Event,
    scroll: html::onscroll::Event,
    // seeked: html::?::Event,
    // seeking: html::?::Event,
    // select: html::?::Event,
    // show: html::?::Event,
    // sort: html::?::Event,
    // stalled: html::?::Event,
    submit: html::onsubmit::Event,
    // suspend: html::?::Event,
    // timeupdate: html::?::Event,
    // toggle: html::?::Event,
    // volumechange: html::?::Event,
    // waiting: html::?::Event,
}

impl<COMP: Component> Display for Events<COMP> {
    fn fmt(&self, _f: &mut Formatter) -> Result<(), Error> {
        Ok(())
    }
}

/// Wrapper type for closures as event handlers.
pub struct EFn<F, E, COMP: Component>(Option<F>, PhantomData<E>, PhantomData<COMP>);

impl<F, E, COMP: Component> EFn<F, E, COMP>
where
    F: FnMut(E) -> COMP::Message + 'static,
{
    pub fn new(f: F) -> Self {
        EFn(Some(f), PhantomData)
    }
}

impl<F, E, COMP: Component> From<F> for Box<dyn EventHandler<Yew<COMP>, E>>
where
    F: FnMut(E) -> COMP::Message + 'static,
    E: 'static,
{
    fn from(f: F) -> Self {
        Box::new(EFn::new(f))
    }
}

impl<F, E, COMP: Component> EventHandler<Yew<COMP>, E> for EFn<F, E, COMP>
where
    F: FnMut(E) -> COMP::Message + 'static,
    E: 'static,
{
    fn attach(&mut self, target: &mut <Yew<COMP> as OutputType>::EventTarget) -> EventListenerHandle {
        let handler = self.0.take().unwrap();
        target.add_event_listener(handler)
    }

    fn render(&self) -> Option<String> {
        None
    }
}

impl<COMP: Component> Yew<COMP> {
    // pub fn install_handlers(target: &mut VTag<COMP>, handlers: &mut Events<COMP>) {
    //     for_events_yew!(handler in handlers => {
    //         target.add_listener(handler);
    //     });
    // }

    // pub fn convert_listener() -> {}

    // pub fn build(
    //     document: &web::Document,
    //     vnode: VNode<'_, Yew<COMP>>,
    // ) -> Result<web::Node, web::error::InvalidCharacterError> {
    //     match vnode {
    //         VNode::Text(text) => Ok(document.create_text_node(&text).into()),
    //         VNode::UnsafeText(text) => Ok(document.create_text_node(&text).into()),
    //         VNode::Element(element) => {
    //             let mut node = document.create_element(element.name)?;
    //             for (key, value) in element.attributes {
    //                 node.set_attribute(&key, &value)?;
    //             }
    //             Yew::<COMP>::install_handlers(&mut node, element.events);
    //             for child in element.children {
    //                 let child_node = Yew::<COMP>::build(document, child)?;
    //                 node.append_child(&child_node);
    //             }
    //             Ok(node.into())
    //         }
    //     }
    // }
    pub fn to_yew_html(vnode: DomVNode<'_, Yew<COMP>>) -> Html<COMP> {
        let node: Option<VNode<COMP>> = match vnode {
            DomVNode::Text(text) => Some(VText::new(text.to_owned()).into()),
            DomVNode::UnsafeText(text) => Some(VText::new(text.to_owned()).into()),
            DomVNode::Element(element) => {
                let mut tag = VTag::new(element.name);
                tag.attributes = element.attributes.into_iter().map(|(k, v)| (k.to_owned(), v)).collect();
                // Yew::<COMP>::install_handlers(&mut tag, element.events);
                Some(tag.into())
            },
        };
        node.unwrap()
        // VNode::<COMP>::VTag(VTag::<COMP>::new("br"))
    }
}
