use std::fmt::{Display, Error, Formatter};
use std::marker::PhantomData;

use yew::html;
use yew::html::{Component, Html, Renderable};
use yew::virtual_dom::vtag::VTag;
use yew::virtual_dom::vtext::VText;
use yew::virtual_dom::Listener;

use crate::dom::VNode as DomVNode;
use crate::events::EventHandler;
use crate::OutputType;

/// DOM output using the stdweb crate
pub struct Yew<COMP: Component + Renderable<COMP>> {
    component_type: PhantomData<COMP>,
}

impl<COMP: Component + Renderable<COMP>> OutputType for Yew<COMP> {
    type Events = Events<COMP>;
    type EventTarget = VTag<COMP>;
    type EventListenerHandle = ();
}

macro_rules! declare_events_yew {
    ($($name:ident : $action:ident ,)*) => {
        /// Container type for DOM events.
        pub struct Events<COMP: Component + Renderable<COMP>> {
            $(
                pub $name: Option<Box<dyn EventHandler<Yew<COMP>, html::$action::Event>>>,
            )*
        }

        $(
            impl ConcreteEvent for html::$action::Event {
                const EVENT_TYPE: &'static str = stringify!($name);
            }

            impl<F, COMP> From<F> for BoxedListener<COMP, html::$action::Event>
            where
                COMP: Component + Renderable<COMP>,
                F: Fn(html::$action::Event) -> COMP::Message + 'static,
            {
                fn from(f: F) -> Self {
                    BoxedListener(Some(Box::new(html::$action::Wrapper::from(f))), PhantomData)
                }
            }

            impl<F, COMP> From<F> for Box<dyn EventHandler<Yew<COMP>, html::$action::Event>>
            where
                F: Fn(html::$action::Event) -> COMP::Message + 'static,
                COMP: Component + Renderable<COMP>,
            {
                fn from(f: F) -> Self {
                    Box::new(BoxedListener::from(f))
                }
            }
        )*

        impl<COMP: Component + Renderable<COMP>> Default for Events<COMP> {
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
// the ones I've been unable to match to yew event types commented out.
//
// This needs review.

declare_events_yew! {
    // abort: ?,
    // autocomplete: ?,
    // autocompleteerror: ?,
    blur: onblur,
    // cancel: ?,
    // canplay: ?,
    // canplaythrough: ?,
    change: onchange,
    click: onclick,
    // close: ?,
    contextmenu: oncontextmenu,
    // cuechange: ?,
    dblclick: ondoubleclick,
    drag: ondrag,
    dragend: ondragend,
    dragenter: ondragenter,
    dragexit: ondragexit,
    dragleave: ondragleave,
    dragover: ondragover,
    dragstart: ondragstart,
    drop: ondrop,
    // durationchange: ?,
    // emptied: ?,
    // ended: ?,
    // error: ?,
    focus: onfocus,
    // ?: ongotpointercapture,
    input: oninput,
    // invalid: ?,
    keydown: onkeydown,
    keypress: onkeypress,
    keyup: onkeyup,
    // load: ?,
    // loadeddata: ?,
    // loadedmetadata: ?,
    // loadstart: ?,
    // ?: onlostpointercapture,
    mousedown: onmousedown,
    mouseenter: onmouseenter,
    mouseleave: onmouseleave,
    mousemove: onmousemove,
    mouseout: onmouseout,
    mouseover: onmouseover,
    mouseup: onmouseup,
    mousewheel: onmousewheel,
    // pause: ?,
    // play: ?,
    // playing: ?,
    // ?: onpointercancel,
    // ?: onpointerdown,
    // ?: onpointerenter,
    // ?: onpointerleave,
    // ?: onpointermove,
    // ?: onpointerout,
    // ?: onpointerover,
    // ?: onpointerup,
    // progress: ?,
    // ratechange: ?,
    // reset: ?,
    // resize: ?,
    scroll: onscroll,
    // seeked: ?,
    // seeking: ?,
    // select: ?,
    // show: ?,
    // sort: ?,
    // stalled: ?,
    submit: onsubmit,
    // suspend: ?,
    // timeupdate: ?,
    // toggle: ?,
    // volumechange: ?,
    // waiting: ?,
}

impl<COMP: Component + Renderable<COMP>> Display for Events<COMP> {
    fn fmt(&self, _f: &mut Formatter) -> Result<(), Error> {
        Ok(())
    }
}

/// A trait representing a concrete event type.
/// Stolen from stdweb: https://docs.rs/stdweb/0.4.15/stdweb/web/event/trait.ConcreteEvent.html
pub trait ConcreteEvent {
    /// A string representing the event type.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Event/type)
    const EVENT_TYPE: &'static str;
}

pub struct BoxedListener<COMP: Component + Renderable<COMP>, E: ConcreteEvent>(
    Option<Box<dyn Listener<COMP>>>,
    PhantomData<E>,
);

impl<E, COMP> EventHandler<Yew<COMP>, E> for BoxedListener<COMP, E>
where
    E: ConcreteEvent,
    COMP: Component + Renderable<COMP>,
{
    fn attach(&mut self, target: &mut <Yew<COMP> as OutputType>::EventTarget) -> () {
        let handler = self.0.take().unwrap();
        target.add_listener(handler)
    }

    fn render(&self) -> Option<String> {
        None
    }
}

impl<COMP: Component + Renderable<COMP>> Yew<COMP> {
    pub fn install_handlers(target: &mut VTag<COMP>, handlers: &mut Events<COMP>) {
        for_events_yew!(handler in handlers => {
            handler.attach(target);
        });
    }

    pub fn build(vnode: DomVNode<'_, Yew<COMP>>) -> Html<COMP> {
        match vnode {
            DomVNode::Text(text) => VText::new(text.to_owned()).into(),
            DomVNode::UnsafeText(text) => VText::new(text.to_owned()).into(),
            DomVNode::Element(element) => {
                let mut tag = VTag::new(element.name);
                tag.attributes = element
                    .attributes
                    .into_iter()
                    .map(|(k, v)| (k.to_owned(), v))
                    .collect();
                Yew::<COMP>::install_handlers(&mut tag, element.events);
                for child in element.children {
                    tag.add_child(Yew::<COMP>::build(child))
                }
                tag.into()
            }
        }
    }
}
