// 🤖 This file is generated!

use crate::*;
/// The `<bdi>` element's builder.
#[derive(Clone, Default, Debug)]
pub struct BidirectionalIsolateBuilder {
    pub element: BidirectionalIsolate,
}
impl BidirectionalIsolateBuilder {
    pub fn build(self) -> BidirectionalIsolate {
        self.element
    }
    pub fn id<T: Into<CowStr>>(mut self, id: T) -> Self {
        let id = id.into();
        debug_assert!(check_id(& id), "Invalid id: {id:?}");
        self.element.id = Some(id);
        self
    }
    pub fn class<T: Into<CowStr>>(mut self, class: T) -> Self {
        let class = class.into();
        debug_assert!(check_class(& class), "Invalid class: {class:?}");
        self.element.classes.insert(class);
        self
    }
    pub fn classes<T: Into<CowStr>, I: IntoIterator<Item = T>>(
        mut self,
        classes: I,
    ) -> Self {
        self.element
            .classes
            .extend(
                classes
                    .into_iter()
                    .map(|class| {
                        let class = class.into();
                        debug_assert!(check_class(& class), "Invalid class: {class:?}");
                        class
                    }),
            );
        self
    }
    pub fn data<K: Into<CowStr>, V: Into<AttributeType>>(
        mut self,
        key: K,
        value: V,
    ) -> Self {
        let key = key.into();
        let value = value.into();
        debug_assert!(check_attribute_name(& key), "Invalid attribute name: {key:?}");
        debug_assert!(
            check_attribute_value(& value), "Invalid attribute value: {value:?}"
        );
        self.element.datas.insert(key.into(), value.into());
        self
    }
    pub fn datas<
        K: Into<CowStr>,
        V: Into<AttributeType>,
        I: IntoIterator<Item = (K, V)>,
    >(mut self, datas: I) -> Self {
        self.element
            .datas
            .extend(
                datas
                    .into_iter()
                    .map(|(key, value)| {
                        let key = key.into();
                        let value = value.into();
                        debug_assert!(
                            check_attribute_name(& key),
                            "Invalid attribute name: {key:?}"
                        );
                        debug_assert!(
                            check_attribute_value(& value),
                            "Invalid attribute value: {value:?}"
                        );
                        (key, value)
                    }),
            );
        self
    }
    pub fn child<T: Into<children::BidirectionalIsolateChild>>(
        mut self,
        child: T,
    ) -> Self {
        self.element.children.push(child.into());
        self
    }
    pub fn children<
        T: Into<children::BidirectionalIsolateChild>,
        I: IntoIterator<Item = T>,
    >(mut self, children: I) -> Self {
        self.element.children.extend(children.into_iter().map(|child| child.into()));
        self
    }
    pub fn accesskey<T: Into<CowStr>>(mut self, accesskey: T) -> Self {
        let value = accesskey.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.accesskey = Some(value);
        self
    }
    pub fn autocapitalize<T: Into<CowStr>>(mut self, autocapitalize: T) -> Self {
        let value = autocapitalize.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.autocapitalize = Some(value);
        self
    }
    pub fn autofocus<T: Into<bool>>(mut self, autofocus: T) -> Self {
        self.element.autofocus = autofocus.into();
        self
    }
    pub fn contenteditable<T: Into<CowStr>>(mut self, contenteditable: T) -> Self {
        let value = contenteditable.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.contenteditable = Some(value);
        self
    }
    pub fn dir<T: Into<CowStr>>(mut self, dir: T) -> Self {
        let value = dir.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.dir = Some(value);
        self
    }
    pub fn draggable<T: Into<bool>>(mut self, draggable: T) -> Self {
        self.element.draggable = draggable.into();
        self
    }
    pub fn enterkeyhint<T: Into<CowStr>>(mut self, enterkeyhint: T) -> Self {
        let value = enterkeyhint.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.enterkeyhint = Some(value);
        self
    }
    pub fn hidden<T: Into<BoolOrF64OrString>>(mut self, hidden: T) -> Self {
        self.element.hidden = hidden.into();
        self
    }
    pub fn inert<T: Into<bool>>(mut self, inert: T) -> Self {
        self.element.inert = inert.into();
        self
    }
    pub fn inputmode<T: Into<CowStr>>(mut self, inputmode: T) -> Self {
        let value = inputmode.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.inputmode = Some(value);
        self
    }
    pub fn is<T: Into<CowStr>>(mut self, is: T) -> Self {
        let value = is.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.is = Some(value);
        self
    }
    pub fn itemid<T: Into<CowStr>>(mut self, itemid: T) -> Self {
        let value = itemid.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.itemid = Some(value);
        self
    }
    pub fn itemprop<T: Into<CowStr>>(mut self, itemprop: T) -> Self {
        let value = itemprop.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.itemprop = Some(value);
        self
    }
    pub fn itemref<T: Into<CowStr>>(mut self, itemref: T) -> Self {
        let value = itemref.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.itemref = Some(value);
        self
    }
    pub fn itemscope<T: Into<CowStr>>(mut self, itemscope: T) -> Self {
        let value = itemscope.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.itemscope = Some(value);
        self
    }
    pub fn itemtype<T: Into<CowStr>>(mut self, itemtype: T) -> Self {
        let value = itemtype.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.itemtype = Some(value);
        self
    }
    pub fn lang<T: Into<CowStr>>(mut self, lang: T) -> Self {
        let value = lang.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.lang = Some(value);
        self
    }
    pub fn nonce<T: Into<CowStr>>(mut self, nonce: T) -> Self {
        let value = nonce.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.nonce = Some(value);
        self
    }
    pub fn popover<T: Into<CowStr>>(mut self, popover: T) -> Self {
        let value = popover.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.popover = Some(value);
        self
    }
    pub fn slot<T: Into<CowStr>>(mut self, slot: T) -> Self {
        let value = slot.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.slot = Some(value);
        self
    }
    pub fn spellcheck<T: Into<bool>>(mut self, spellcheck: T) -> Self {
        self.element.spellcheck = spellcheck.into();
        self
    }
    pub fn style<T: Into<CowStr>>(mut self, style: T) -> Self {
        let value = style.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.style = Some(value);
        self
    }
    pub fn tabindex<T: Into<i32>>(mut self, tabindex: T) -> Self {
        let value = tabindex.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.tabindex = Some(value);
        self
    }
    pub fn title<T: Into<CowStr>>(mut self, title: T) -> Self {
        let value = title.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.title = Some(value);
        self
    }
    pub fn translate<T: Into<bool>>(mut self, translate: T) -> Self {
        self.element.translate = translate.into();
        self
    }
    pub fn on_auxclick<T: Into<CowStr>>(mut self, on_auxclick: T) -> Self {
        let value = on_auxclick.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_auxclick = Some(value);
        self
    }
    pub fn on_beforeinput<T: Into<CowStr>>(mut self, on_beforeinput: T) -> Self {
        let value = on_beforeinput.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_beforeinput = Some(value);
        self
    }
    pub fn on_beforematch<T: Into<CowStr>>(mut self, on_beforematch: T) -> Self {
        let value = on_beforematch.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_beforematch = Some(value);
        self
    }
    pub fn on_beforetoggle<T: Into<CowStr>>(mut self, on_beforetoggle: T) -> Self {
        let value = on_beforetoggle.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_beforetoggle = Some(value);
        self
    }
    pub fn on_blur<T: Into<CowStr>>(mut self, on_blur: T) -> Self {
        let value = on_blur.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_blur = Some(value);
        self
    }
    pub fn on_cancel<T: Into<CowStr>>(mut self, on_cancel: T) -> Self {
        let value = on_cancel.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_cancel = Some(value);
        self
    }
    pub fn on_canplay<T: Into<CowStr>>(mut self, on_canplay: T) -> Self {
        let value = on_canplay.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_canplay = Some(value);
        self
    }
    pub fn on_canplaythrough<T: Into<CowStr>>(mut self, on_canplaythrough: T) -> Self {
        let value = on_canplaythrough.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_canplaythrough = Some(value);
        self
    }
    pub fn on_change<T: Into<CowStr>>(mut self, on_change: T) -> Self {
        let value = on_change.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_change = Some(value);
        self
    }
    pub fn on_click<T: Into<CowStr>>(mut self, on_click: T) -> Self {
        let value = on_click.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_click = Some(value);
        self
    }
    pub fn on_close<T: Into<CowStr>>(mut self, on_close: T) -> Self {
        let value = on_close.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_close = Some(value);
        self
    }
    pub fn on_contextlost<T: Into<CowStr>>(mut self, on_contextlost: T) -> Self {
        let value = on_contextlost.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_contextlost = Some(value);
        self
    }
    pub fn on_contextmenu<T: Into<CowStr>>(mut self, on_contextmenu: T) -> Self {
        let value = on_contextmenu.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_contextmenu = Some(value);
        self
    }
    pub fn on_contextrestored<T: Into<CowStr>>(mut self, on_contextrestored: T) -> Self {
        let value = on_contextrestored.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_contextrestored = Some(value);
        self
    }
    pub fn on_copy<T: Into<CowStr>>(mut self, on_copy: T) -> Self {
        let value = on_copy.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_copy = Some(value);
        self
    }
    pub fn on_cuechange<T: Into<CowStr>>(mut self, on_cuechange: T) -> Self {
        let value = on_cuechange.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_cuechange = Some(value);
        self
    }
    pub fn on_cut<T: Into<CowStr>>(mut self, on_cut: T) -> Self {
        let value = on_cut.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_cut = Some(value);
        self
    }
    pub fn on_dblclick<T: Into<CowStr>>(mut self, on_dblclick: T) -> Self {
        let value = on_dblclick.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_dblclick = Some(value);
        self
    }
    pub fn on_drag<T: Into<CowStr>>(mut self, on_drag: T) -> Self {
        let value = on_drag.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_drag = Some(value);
        self
    }
    pub fn on_dragend<T: Into<CowStr>>(mut self, on_dragend: T) -> Self {
        let value = on_dragend.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_dragend = Some(value);
        self
    }
    pub fn on_dragenter<T: Into<CowStr>>(mut self, on_dragenter: T) -> Self {
        let value = on_dragenter.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_dragenter = Some(value);
        self
    }
    pub fn on_dragleave<T: Into<CowStr>>(mut self, on_dragleave: T) -> Self {
        let value = on_dragleave.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_dragleave = Some(value);
        self
    }
    pub fn on_dragover<T: Into<CowStr>>(mut self, on_dragover: T) -> Self {
        let value = on_dragover.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_dragover = Some(value);
        self
    }
    pub fn on_dragstart<T: Into<CowStr>>(mut self, on_dragstart: T) -> Self {
        let value = on_dragstart.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_dragstart = Some(value);
        self
    }
    pub fn on_drop<T: Into<CowStr>>(mut self, on_drop: T) -> Self {
        let value = on_drop.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_drop = Some(value);
        self
    }
    pub fn on_durationchange<T: Into<CowStr>>(mut self, on_durationchange: T) -> Self {
        let value = on_durationchange.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_durationchange = Some(value);
        self
    }
    pub fn on_emptied<T: Into<CowStr>>(mut self, on_emptied: T) -> Self {
        let value = on_emptied.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_emptied = Some(value);
        self
    }
    pub fn on_ended<T: Into<CowStr>>(mut self, on_ended: T) -> Self {
        let value = on_ended.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_ended = Some(value);
        self
    }
    pub fn on_error<T: Into<CowStr>>(mut self, on_error: T) -> Self {
        let value = on_error.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_error = Some(value);
        self
    }
    pub fn on_focus<T: Into<CowStr>>(mut self, on_focus: T) -> Self {
        let value = on_focus.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_focus = Some(value);
        self
    }
    pub fn on_formdata<T: Into<CowStr>>(mut self, on_formdata: T) -> Self {
        let value = on_formdata.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_formdata = Some(value);
        self
    }
    pub fn on_input<T: Into<CowStr>>(mut self, on_input: T) -> Self {
        let value = on_input.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_input = Some(value);
        self
    }
    pub fn on_invalid<T: Into<CowStr>>(mut self, on_invalid: T) -> Self {
        let value = on_invalid.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_invalid = Some(value);
        self
    }
    pub fn on_keydown<T: Into<CowStr>>(mut self, on_keydown: T) -> Self {
        let value = on_keydown.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_keydown = Some(value);
        self
    }
    pub fn on_keypress<T: Into<CowStr>>(mut self, on_keypress: T) -> Self {
        let value = on_keypress.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_keypress = Some(value);
        self
    }
    pub fn on_keyup<T: Into<CowStr>>(mut self, on_keyup: T) -> Self {
        let value = on_keyup.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_keyup = Some(value);
        self
    }
    pub fn on_load<T: Into<CowStr>>(mut self, on_load: T) -> Self {
        let value = on_load.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_load = Some(value);
        self
    }
    pub fn on_loadeddata<T: Into<CowStr>>(mut self, on_loadeddata: T) -> Self {
        let value = on_loadeddata.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_loadeddata = Some(value);
        self
    }
    pub fn on_loadedmetadata<T: Into<CowStr>>(mut self, on_loadedmetadata: T) -> Self {
        let value = on_loadedmetadata.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_loadedmetadata = Some(value);
        self
    }
    pub fn on_loadstart<T: Into<CowStr>>(mut self, on_loadstart: T) -> Self {
        let value = on_loadstart.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_loadstart = Some(value);
        self
    }
    pub fn on_mousedown<T: Into<CowStr>>(mut self, on_mousedown: T) -> Self {
        let value = on_mousedown.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_mousedown = Some(value);
        self
    }
    pub fn on_mouseenter<T: Into<CowStr>>(mut self, on_mouseenter: T) -> Self {
        let value = on_mouseenter.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_mouseenter = Some(value);
        self
    }
    pub fn on_mouseleave<T: Into<CowStr>>(mut self, on_mouseleave: T) -> Self {
        let value = on_mouseleave.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_mouseleave = Some(value);
        self
    }
    pub fn on_mousemove<T: Into<CowStr>>(mut self, on_mousemove: T) -> Self {
        let value = on_mousemove.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_mousemove = Some(value);
        self
    }
    pub fn on_mouseout<T: Into<CowStr>>(mut self, on_mouseout: T) -> Self {
        let value = on_mouseout.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_mouseout = Some(value);
        self
    }
    pub fn on_mouseover<T: Into<CowStr>>(mut self, on_mouseover: T) -> Self {
        let value = on_mouseover.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_mouseover = Some(value);
        self
    }
    pub fn on_mouseup<T: Into<CowStr>>(mut self, on_mouseup: T) -> Self {
        let value = on_mouseup.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_mouseup = Some(value);
        self
    }
    pub fn on_paste<T: Into<CowStr>>(mut self, on_paste: T) -> Self {
        let value = on_paste.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_paste = Some(value);
        self
    }
    pub fn on_pause<T: Into<CowStr>>(mut self, on_pause: T) -> Self {
        let value = on_pause.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_pause = Some(value);
        self
    }
    pub fn on_play<T: Into<CowStr>>(mut self, on_play: T) -> Self {
        let value = on_play.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_play = Some(value);
        self
    }
    pub fn on_playing<T: Into<CowStr>>(mut self, on_playing: T) -> Self {
        let value = on_playing.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_playing = Some(value);
        self
    }
    pub fn on_progress<T: Into<CowStr>>(mut self, on_progress: T) -> Self {
        let value = on_progress.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_progress = Some(value);
        self
    }
    pub fn on_ratechange<T: Into<CowStr>>(mut self, on_ratechange: T) -> Self {
        let value = on_ratechange.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_ratechange = Some(value);
        self
    }
    pub fn on_reset<T: Into<CowStr>>(mut self, on_reset: T) -> Self {
        let value = on_reset.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_reset = Some(value);
        self
    }
    pub fn on_resize<T: Into<CowStr>>(mut self, on_resize: T) -> Self {
        let value = on_resize.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_resize = Some(value);
        self
    }
    pub fn on_scroll<T: Into<CowStr>>(mut self, on_scroll: T) -> Self {
        let value = on_scroll.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_scroll = Some(value);
        self
    }
    pub fn on_scrollend<T: Into<CowStr>>(mut self, on_scrollend: T) -> Self {
        let value = on_scrollend.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_scrollend = Some(value);
        self
    }
    pub fn on_securitypolicyviolation<T: Into<CowStr>>(
        mut self,
        on_securitypolicyviolation: T,
    ) -> Self {
        let value = on_securitypolicyviolation.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_securitypolicyviolation = Some(value);
        self
    }
    pub fn on_seeked<T: Into<CowStr>>(mut self, on_seeked: T) -> Self {
        let value = on_seeked.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_seeked = Some(value);
        self
    }
    pub fn on_seeking<T: Into<CowStr>>(mut self, on_seeking: T) -> Self {
        let value = on_seeking.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_seeking = Some(value);
        self
    }
    pub fn on_select<T: Into<CowStr>>(mut self, on_select: T) -> Self {
        let value = on_select.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_select = Some(value);
        self
    }
    pub fn on_slotchange<T: Into<CowStr>>(mut self, on_slotchange: T) -> Self {
        let value = on_slotchange.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_slotchange = Some(value);
        self
    }
    pub fn on_stalled<T: Into<CowStr>>(mut self, on_stalled: T) -> Self {
        let value = on_stalled.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_stalled = Some(value);
        self
    }
    pub fn on_submit<T: Into<CowStr>>(mut self, on_submit: T) -> Self {
        let value = on_submit.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_submit = Some(value);
        self
    }
    pub fn on_suspend<T: Into<CowStr>>(mut self, on_suspend: T) -> Self {
        let value = on_suspend.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_suspend = Some(value);
        self
    }
    pub fn on_timeupdate<T: Into<CowStr>>(mut self, on_timeupdate: T) -> Self {
        let value = on_timeupdate.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_timeupdate = Some(value);
        self
    }
    pub fn on_toggle<T: Into<CowStr>>(mut self, on_toggle: T) -> Self {
        let value = on_toggle.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_toggle = Some(value);
        self
    }
    pub fn on_volumechange<T: Into<CowStr>>(mut self, on_volumechange: T) -> Self {
        let value = on_volumechange.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_volumechange = Some(value);
        self
    }
    pub fn on_waiting<T: Into<CowStr>>(mut self, on_waiting: T) -> Self {
        let value = on_waiting.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_waiting = Some(value);
        self
    }
    pub fn on_wheel<T: Into<CowStr>>(mut self, on_wheel: T) -> Self {
        let value = on_wheel.into();
        debug_assert!(
            check_attribute_value(& AttributeType::from(value.clone())),
            "Invalid attribute value: {value:?}"
        );
        self.element.on_wheel = Some(value);
        self
    }
}
impl From<BidirectionalIsolate> for BidirectionalIsolateBuilder {
    fn from(element: BidirectionalIsolate) -> Self {
        Self { element }
    }
}
impl std::fmt::Display for BidirectionalIsolateBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.element)
    }
}
