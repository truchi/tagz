// 🤖 This file is generated!

use crate::*;
/// An autonomous custom element.
///
/// [`MDN`](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements)
#[derive(Clone)]
pub struct Custom {
    pub tag: CowStr,
    pub id: StdOption<CowStr>,
    pub classes: HashSet<CowStr>,
    pub datas: HashMap<CowStr, AttributeType>,
    pub accesskey: std::option::Option<CowStr>,
    pub autocapitalize: std::option::Option<CowStr>,
    pub autofocus: bool,
    pub contenteditable: std::option::Option<CowStr>,
    pub dir: std::option::Option<CowStr>,
    pub disabled: std::option::Option<CowStr>,
    pub draggable: bool,
    pub enterkeyhint: std::option::Option<CowStr>,
    pub form: std::option::Option<CowStr>,
    pub hidden: BoolOrF64OrString,
    pub inert: bool,
    pub inputmode: std::option::Option<CowStr>,
    pub is: std::option::Option<CowStr>,
    pub itemid: std::option::Option<CowStr>,
    pub itemprop: std::option::Option<CowStr>,
    pub itemref: std::option::Option<CowStr>,
    pub itemscope: std::option::Option<CowStr>,
    pub itemtype: std::option::Option<CowStr>,
    pub lang: std::option::Option<CowStr>,
    pub name: std::option::Option<CowStr>,
    pub nonce: std::option::Option<CowStr>,
    pub popover: std::option::Option<CowStr>,
    pub readonly: std::option::Option<CowStr>,
    pub slot: std::option::Option<CowStr>,
    pub spellcheck: bool,
    pub style: std::option::Option<CowStr>,
    pub tabindex: std::option::Option<i32>,
    pub title: std::option::Option<CowStr>,
    pub translate: bool,
    pub on_auxclick: std::option::Option<CowStr>,
    pub on_beforeinput: std::option::Option<CowStr>,
    pub on_beforematch: std::option::Option<CowStr>,
    pub on_beforetoggle: std::option::Option<CowStr>,
    pub on_blur: std::option::Option<CowStr>,
    pub on_cancel: std::option::Option<CowStr>,
    pub on_canplay: std::option::Option<CowStr>,
    pub on_canplaythrough: std::option::Option<CowStr>,
    pub on_change: std::option::Option<CowStr>,
    pub on_click: std::option::Option<CowStr>,
    pub on_close: std::option::Option<CowStr>,
    pub on_contextlost: std::option::Option<CowStr>,
    pub on_contextmenu: std::option::Option<CowStr>,
    pub on_contextrestored: std::option::Option<CowStr>,
    pub on_copy: std::option::Option<CowStr>,
    pub on_cuechange: std::option::Option<CowStr>,
    pub on_cut: std::option::Option<CowStr>,
    pub on_dblclick: std::option::Option<CowStr>,
    pub on_drag: std::option::Option<CowStr>,
    pub on_dragend: std::option::Option<CowStr>,
    pub on_dragenter: std::option::Option<CowStr>,
    pub on_dragleave: std::option::Option<CowStr>,
    pub on_dragover: std::option::Option<CowStr>,
    pub on_dragstart: std::option::Option<CowStr>,
    pub on_drop: std::option::Option<CowStr>,
    pub on_durationchange: std::option::Option<CowStr>,
    pub on_emptied: std::option::Option<CowStr>,
    pub on_ended: std::option::Option<CowStr>,
    pub on_error: std::option::Option<CowStr>,
    pub on_focus: std::option::Option<CowStr>,
    pub on_formdata: std::option::Option<CowStr>,
    pub on_input: std::option::Option<CowStr>,
    pub on_invalid: std::option::Option<CowStr>,
    pub on_keydown: std::option::Option<CowStr>,
    pub on_keypress: std::option::Option<CowStr>,
    pub on_keyup: std::option::Option<CowStr>,
    pub on_load: std::option::Option<CowStr>,
    pub on_loadeddata: std::option::Option<CowStr>,
    pub on_loadedmetadata: std::option::Option<CowStr>,
    pub on_loadstart: std::option::Option<CowStr>,
    pub on_mousedown: std::option::Option<CowStr>,
    pub on_mouseenter: std::option::Option<CowStr>,
    pub on_mouseleave: std::option::Option<CowStr>,
    pub on_mousemove: std::option::Option<CowStr>,
    pub on_mouseout: std::option::Option<CowStr>,
    pub on_mouseover: std::option::Option<CowStr>,
    pub on_mouseup: std::option::Option<CowStr>,
    pub on_paste: std::option::Option<CowStr>,
    pub on_pause: std::option::Option<CowStr>,
    pub on_play: std::option::Option<CowStr>,
    pub on_playing: std::option::Option<CowStr>,
    pub on_progress: std::option::Option<CowStr>,
    pub on_ratechange: std::option::Option<CowStr>,
    pub on_reset: std::option::Option<CowStr>,
    pub on_resize: std::option::Option<CowStr>,
    pub on_scroll: std::option::Option<CowStr>,
    pub on_scrollend: std::option::Option<CowStr>,
    pub on_securitypolicyviolation: std::option::Option<CowStr>,
    pub on_seeked: std::option::Option<CowStr>,
    pub on_seeking: std::option::Option<CowStr>,
    pub on_select: std::option::Option<CowStr>,
    pub on_slotchange: std::option::Option<CowStr>,
    pub on_stalled: std::option::Option<CowStr>,
    pub on_submit: std::option::Option<CowStr>,
    pub on_suspend: std::option::Option<CowStr>,
    pub on_timeupdate: std::option::Option<CowStr>,
    pub on_toggle: std::option::Option<CowStr>,
    pub on_volumechange: std::option::Option<CowStr>,
    pub on_waiting: std::option::Option<CowStr>,
    pub on_wheel: std::option::Option<CowStr>,
    pub children: Vec<children::CustomChild>,
}
impl Custom {
    pub fn new<T: Into<CowStr>>(tag: T) -> builders::CustomBuilder {
        builders::CustomBuilder {
            element: Custom {
                tag: tag.into(),
                id: None,
                classes: HashSet::new(),
                datas: HashMap::new(),
                accesskey: Default::default(),
                autocapitalize: Default::default(),
                autofocus: Default::default(),
                contenteditable: Default::default(),
                dir: Default::default(),
                disabled: Default::default(),
                draggable: Default::default(),
                enterkeyhint: Default::default(),
                form: Default::default(),
                hidden: Default::default(),
                inert: Default::default(),
                inputmode: Default::default(),
                is: Default::default(),
                itemid: Default::default(),
                itemprop: Default::default(),
                itemref: Default::default(),
                itemscope: Default::default(),
                itemtype: Default::default(),
                lang: Default::default(),
                name: Default::default(),
                nonce: Default::default(),
                popover: Default::default(),
                readonly: Default::default(),
                slot: Default::default(),
                spellcheck: Default::default(),
                style: Default::default(),
                tabindex: Default::default(),
                title: Default::default(),
                translate: Default::default(),
                on_auxclick: Default::default(),
                on_beforeinput: Default::default(),
                on_beforematch: Default::default(),
                on_beforetoggle: Default::default(),
                on_blur: Default::default(),
                on_cancel: Default::default(),
                on_canplay: Default::default(),
                on_canplaythrough: Default::default(),
                on_change: Default::default(),
                on_click: Default::default(),
                on_close: Default::default(),
                on_contextlost: Default::default(),
                on_contextmenu: Default::default(),
                on_contextrestored: Default::default(),
                on_copy: Default::default(),
                on_cuechange: Default::default(),
                on_cut: Default::default(),
                on_dblclick: Default::default(),
                on_drag: Default::default(),
                on_dragend: Default::default(),
                on_dragenter: Default::default(),
                on_dragleave: Default::default(),
                on_dragover: Default::default(),
                on_dragstart: Default::default(),
                on_drop: Default::default(),
                on_durationchange: Default::default(),
                on_emptied: Default::default(),
                on_ended: Default::default(),
                on_error: Default::default(),
                on_focus: Default::default(),
                on_formdata: Default::default(),
                on_input: Default::default(),
                on_invalid: Default::default(),
                on_keydown: Default::default(),
                on_keypress: Default::default(),
                on_keyup: Default::default(),
                on_load: Default::default(),
                on_loadeddata: Default::default(),
                on_loadedmetadata: Default::default(),
                on_loadstart: Default::default(),
                on_mousedown: Default::default(),
                on_mouseenter: Default::default(),
                on_mouseleave: Default::default(),
                on_mousemove: Default::default(),
                on_mouseout: Default::default(),
                on_mouseover: Default::default(),
                on_mouseup: Default::default(),
                on_paste: Default::default(),
                on_pause: Default::default(),
                on_play: Default::default(),
                on_playing: Default::default(),
                on_progress: Default::default(),
                on_ratechange: Default::default(),
                on_reset: Default::default(),
                on_resize: Default::default(),
                on_scroll: Default::default(),
                on_scrollend: Default::default(),
                on_securitypolicyviolation: Default::default(),
                on_seeked: Default::default(),
                on_seeking: Default::default(),
                on_select: Default::default(),
                on_slotchange: Default::default(),
                on_stalled: Default::default(),
                on_submit: Default::default(),
                on_suspend: Default::default(),
                on_timeupdate: Default::default(),
                on_toggle: Default::default(),
                on_volumechange: Default::default(),
                on_waiting: Default::default(),
                on_wheel: Default::default(),
                children: Vec::new(),
            },
        }
    }
}
impl From<builders::CustomBuilder> for Custom {
    fn from(builder: builders::CustomBuilder) -> Self {
        builder.element
    }
}
impl std::fmt::Debug for Custom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct(&format!("<{}>", self.tag));
        if let Some(id) = &self.id {
            f.field("id", &id);
        }
        if !self.classes.is_empty() {
            f.field("classes", &self.classes);
        }
        if !self.datas.is_empty() {
            f.field(
                "datas",
                &self
                    .datas
                    .iter()
                    .map(|(key, value)| {
                        (
                            key,
                            match value {
                                AttributeType::String(value) => {
                                    Box::new(value) as Box<dyn std::fmt::Debug>
                                }
                                AttributeType::Bool(value) => Box::new(value),
                                AttributeType::I16(value) => Box::new(value),
                                AttributeType::U16(value) => Box::new(value),
                                AttributeType::I32(value) => Box::new(value),
                                AttributeType::U32(value) => Box::new(value),
                                AttributeType::F32(value) => Box::new(value),
                                AttributeType::I64(value) => Box::new(value),
                                AttributeType::U64(value) => Box::new(value),
                                AttributeType::F64(value) => Box::new(value),
                            },
                        )
                    })
                    .collect::<HashMap<_, _>>(),
            );
        }
        if let Some(value) = &self.accesskey {
            f.field("accesskey", &value);
        }
        if let Some(value) = &self.autocapitalize {
            f.field("autocapitalize", &value);
        }
        if self.autofocus {
            f.field("autofocus", &true);
        }
        if let Some(value) = &self.contenteditable {
            f.field("contenteditable", &value);
        }
        if let Some(value) = &self.dir {
            f.field("dir", &value);
        }
        if let Some(value) = &self.disabled {
            f.field("disabled", &value);
        }
        if self.draggable {
            f.field("draggable", &true);
        }
        if let Some(value) = &self.enterkeyhint {
            f.field("enterkeyhint", &value);
        }
        if let Some(value) = &self.form {
            f.field("form", &value);
        }
        match &self.hidden {
            BoolOrF64OrString::Bool(false) => &mut f,
            BoolOrF64OrString::Bool(true) => f.field("hidden", &true),
            BoolOrF64OrString::F64(value) => f.field("hidden", &value),
            BoolOrF64OrString::String(value) => f.field("hidden", &value),
        };
        if self.inert {
            f.field("inert", &true);
        }
        if let Some(value) = &self.inputmode {
            f.field("inputmode", &value);
        }
        if let Some(value) = &self.is {
            f.field("is", &value);
        }
        if let Some(value) = &self.itemid {
            f.field("itemid", &value);
        }
        if let Some(value) = &self.itemprop {
            f.field("itemprop", &value);
        }
        if let Some(value) = &self.itemref {
            f.field("itemref", &value);
        }
        if let Some(value) = &self.itemscope {
            f.field("itemscope", &value);
        }
        if let Some(value) = &self.itemtype {
            f.field("itemtype", &value);
        }
        if let Some(value) = &self.lang {
            f.field("lang", &value);
        }
        if let Some(value) = &self.name {
            f.field("name", &value);
        }
        if let Some(value) = &self.nonce {
            f.field("nonce", &value);
        }
        if let Some(value) = &self.popover {
            f.field("popover", &value);
        }
        if let Some(value) = &self.readonly {
            f.field("readonly", &value);
        }
        if let Some(value) = &self.slot {
            f.field("slot", &value);
        }
        if self.spellcheck {
            f.field("spellcheck", &true);
        }
        if let Some(value) = &self.style {
            f.field("style", &value);
        }
        if let Some(value) = &self.tabindex {
            f.field("tabindex", &value);
        }
        if let Some(value) = &self.title {
            f.field("title", &value);
        }
        if self.translate {
            f.field("translate", &true);
        }
        if let Some(value) = &self.on_auxclick {
            f.field("on_auxclick", &value);
        }
        if let Some(value) = &self.on_beforeinput {
            f.field("on_beforeinput", &value);
        }
        if let Some(value) = &self.on_beforematch {
            f.field("on_beforematch", &value);
        }
        if let Some(value) = &self.on_beforetoggle {
            f.field("on_beforetoggle", &value);
        }
        if let Some(value) = &self.on_blur {
            f.field("on_blur", &value);
        }
        if let Some(value) = &self.on_cancel {
            f.field("on_cancel", &value);
        }
        if let Some(value) = &self.on_canplay {
            f.field("on_canplay", &value);
        }
        if let Some(value) = &self.on_canplaythrough {
            f.field("on_canplaythrough", &value);
        }
        if let Some(value) = &self.on_change {
            f.field("on_change", &value);
        }
        if let Some(value) = &self.on_click {
            f.field("on_click", &value);
        }
        if let Some(value) = &self.on_close {
            f.field("on_close", &value);
        }
        if let Some(value) = &self.on_contextlost {
            f.field("on_contextlost", &value);
        }
        if let Some(value) = &self.on_contextmenu {
            f.field("on_contextmenu", &value);
        }
        if let Some(value) = &self.on_contextrestored {
            f.field("on_contextrestored", &value);
        }
        if let Some(value) = &self.on_copy {
            f.field("on_copy", &value);
        }
        if let Some(value) = &self.on_cuechange {
            f.field("on_cuechange", &value);
        }
        if let Some(value) = &self.on_cut {
            f.field("on_cut", &value);
        }
        if let Some(value) = &self.on_dblclick {
            f.field("on_dblclick", &value);
        }
        if let Some(value) = &self.on_drag {
            f.field("on_drag", &value);
        }
        if let Some(value) = &self.on_dragend {
            f.field("on_dragend", &value);
        }
        if let Some(value) = &self.on_dragenter {
            f.field("on_dragenter", &value);
        }
        if let Some(value) = &self.on_dragleave {
            f.field("on_dragleave", &value);
        }
        if let Some(value) = &self.on_dragover {
            f.field("on_dragover", &value);
        }
        if let Some(value) = &self.on_dragstart {
            f.field("on_dragstart", &value);
        }
        if let Some(value) = &self.on_drop {
            f.field("on_drop", &value);
        }
        if let Some(value) = &self.on_durationchange {
            f.field("on_durationchange", &value);
        }
        if let Some(value) = &self.on_emptied {
            f.field("on_emptied", &value);
        }
        if let Some(value) = &self.on_ended {
            f.field("on_ended", &value);
        }
        if let Some(value) = &self.on_error {
            f.field("on_error", &value);
        }
        if let Some(value) = &self.on_focus {
            f.field("on_focus", &value);
        }
        if let Some(value) = &self.on_formdata {
            f.field("on_formdata", &value);
        }
        if let Some(value) = &self.on_input {
            f.field("on_input", &value);
        }
        if let Some(value) = &self.on_invalid {
            f.field("on_invalid", &value);
        }
        if let Some(value) = &self.on_keydown {
            f.field("on_keydown", &value);
        }
        if let Some(value) = &self.on_keypress {
            f.field("on_keypress", &value);
        }
        if let Some(value) = &self.on_keyup {
            f.field("on_keyup", &value);
        }
        if let Some(value) = &self.on_load {
            f.field("on_load", &value);
        }
        if let Some(value) = &self.on_loadeddata {
            f.field("on_loadeddata", &value);
        }
        if let Some(value) = &self.on_loadedmetadata {
            f.field("on_loadedmetadata", &value);
        }
        if let Some(value) = &self.on_loadstart {
            f.field("on_loadstart", &value);
        }
        if let Some(value) = &self.on_mousedown {
            f.field("on_mousedown", &value);
        }
        if let Some(value) = &self.on_mouseenter {
            f.field("on_mouseenter", &value);
        }
        if let Some(value) = &self.on_mouseleave {
            f.field("on_mouseleave", &value);
        }
        if let Some(value) = &self.on_mousemove {
            f.field("on_mousemove", &value);
        }
        if let Some(value) = &self.on_mouseout {
            f.field("on_mouseout", &value);
        }
        if let Some(value) = &self.on_mouseover {
            f.field("on_mouseover", &value);
        }
        if let Some(value) = &self.on_mouseup {
            f.field("on_mouseup", &value);
        }
        if let Some(value) = &self.on_paste {
            f.field("on_paste", &value);
        }
        if let Some(value) = &self.on_pause {
            f.field("on_pause", &value);
        }
        if let Some(value) = &self.on_play {
            f.field("on_play", &value);
        }
        if let Some(value) = &self.on_playing {
            f.field("on_playing", &value);
        }
        if let Some(value) = &self.on_progress {
            f.field("on_progress", &value);
        }
        if let Some(value) = &self.on_ratechange {
            f.field("on_ratechange", &value);
        }
        if let Some(value) = &self.on_reset {
            f.field("on_reset", &value);
        }
        if let Some(value) = &self.on_resize {
            f.field("on_resize", &value);
        }
        if let Some(value) = &self.on_scroll {
            f.field("on_scroll", &value);
        }
        if let Some(value) = &self.on_scrollend {
            f.field("on_scrollend", &value);
        }
        if let Some(value) = &self.on_securitypolicyviolation {
            f.field("on_securitypolicyviolation", &value);
        }
        if let Some(value) = &self.on_seeked {
            f.field("on_seeked", &value);
        }
        if let Some(value) = &self.on_seeking {
            f.field("on_seeking", &value);
        }
        if let Some(value) = &self.on_select {
            f.field("on_select", &value);
        }
        if let Some(value) = &self.on_slotchange {
            f.field("on_slotchange", &value);
        }
        if let Some(value) = &self.on_stalled {
            f.field("on_stalled", &value);
        }
        if let Some(value) = &self.on_submit {
            f.field("on_submit", &value);
        }
        if let Some(value) = &self.on_suspend {
            f.field("on_suspend", &value);
        }
        if let Some(value) = &self.on_timeupdate {
            f.field("on_timeupdate", &value);
        }
        if let Some(value) = &self.on_toggle {
            f.field("on_toggle", &value);
        }
        if let Some(value) = &self.on_volumechange {
            f.field("on_volumechange", &value);
        }
        if let Some(value) = &self.on_waiting {
            f.field("on_waiting", &value);
        }
        if let Some(value) = &self.on_wheel {
            f.field("on_wheel", &value);
        }
        if !self.children.is_empty() {
            f.field("children", &self.children);
        }
        f.finish()
    }
}
impl std::fmt::Display for Custom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}", self.tag)?;
        if let Some(id) = &self.id {
            write!(f, " id='{id}'")?;
        }
        let mut classes = self.classes.iter();
        if let Some(class) = classes.next() {
            write!(f, " class='{class}")?;
            for class in classes {
                write!(f, " {class}")?;
            }
            write!(f, "'")?;
        }
        for (key, value) in &self.datas {
            match value {
                AttributeType::Bool(false) => {}
                AttributeType::Bool(true) => write!(f, " data-{key}")?,
                AttributeType::I16(value) => write!(f, " data-{key}={value}")?,
                AttributeType::U16(value) => write!(f, " data-{key}={value}")?,
                AttributeType::I32(value) => write!(f, " data-{key}={value}")?,
                AttributeType::U32(value) => write!(f, " data-{key}={value}")?,
                AttributeType::F32(value) => write!(f, " data-{key}={value}")?,
                AttributeType::I64(value) => write!(f, " data-{key}={value}")?,
                AttributeType::U64(value) => write!(f, " data-{key}={value}")?,
                AttributeType::F64(value) => write!(f, " data-{key}={value}")?,
                AttributeType::String(value) => write!(f, " data-{key}='{value}'")?,
            }
        }
        if let Some(value) = &self.accesskey {
            write!(f, " {}='{value}'", "accesskey")?;
        }
        if let Some(value) = &self.autocapitalize {
            write!(f, " {}='{value}'", "autocapitalize")?;
        }
        if self.autofocus {
            write!(f, " {}", "autofocus")?;
        }
        if let Some(value) = &self.contenteditable {
            write!(f, " {}='{value}'", "contenteditable")?;
        }
        if let Some(value) = &self.dir {
            write!(f, " {}='{value}'", "dir")?;
        }
        if let Some(value) = &self.disabled {
            write!(f, " {}='{value}'", "disabled")?;
        }
        if self.draggable {
            write!(f, " {}", "draggable")?;
        }
        if let Some(value) = &self.enterkeyhint {
            write!(f, " {}='{value}'", "enterkeyhint")?;
        }
        if let Some(value) = &self.form {
            write!(f, " {}='{value}'", "form")?;
        }
        match &self.hidden {
            BoolOrF64OrString::Bool(false) => {}
            BoolOrF64OrString::Bool(true) => write!(f, " {}", "hidden")?,
            BoolOrF64OrString::F64(value) => write!(f, " {}={value}", "hidden")?,
            BoolOrF64OrString::String(value) => write!(f, " {}='{value}'", "hidden")?,
        }
        if self.inert {
            write!(f, " {}", "inert")?;
        }
        if let Some(value) = &self.inputmode {
            write!(f, " {}='{value}'", "inputmode")?;
        }
        if let Some(value) = &self.is {
            write!(f, " {}='{value}'", "is")?;
        }
        if let Some(value) = &self.itemid {
            write!(f, " {}='{value}'", "itemid")?;
        }
        if let Some(value) = &self.itemprop {
            write!(f, " {}='{value}'", "itemprop")?;
        }
        if let Some(value) = &self.itemref {
            write!(f, " {}='{value}'", "itemref")?;
        }
        if let Some(value) = &self.itemscope {
            write!(f, " {}='{value}'", "itemscope")?;
        }
        if let Some(value) = &self.itemtype {
            write!(f, " {}='{value}'", "itemtype")?;
        }
        if let Some(value) = &self.lang {
            write!(f, " {}='{value}'", "lang")?;
        }
        if let Some(value) = &self.name {
            write!(f, " {}='{value}'", "name")?;
        }
        if let Some(value) = &self.nonce {
            write!(f, " {}='{value}'", "nonce")?;
        }
        if let Some(value) = &self.popover {
            write!(f, " {}='{value}'", "popover")?;
        }
        if let Some(value) = &self.readonly {
            write!(f, " {}='{value}'", "readonly")?;
        }
        if let Some(value) = &self.slot {
            write!(f, " {}='{value}'", "slot")?;
        }
        if self.spellcheck {
            write!(f, " {}", "spellcheck")?;
        }
        if let Some(value) = &self.style {
            write!(f, " {}='{value}'", "style")?;
        }
        if let Some(value) = &self.tabindex {
            write!(f, " {}={value}", "tabindex")?;
        }
        if let Some(value) = &self.title {
            write!(f, " {}='{value}'", "title")?;
        }
        if self.translate {
            write!(f, " {}", "translate")?;
        }
        if let Some(value) = &self.on_auxclick {
            write!(f, " {}='{value}'", "on_auxclick")?;
        }
        if let Some(value) = &self.on_beforeinput {
            write!(f, " {}='{value}'", "on_beforeinput")?;
        }
        if let Some(value) = &self.on_beforematch {
            write!(f, " {}='{value}'", "on_beforematch")?;
        }
        if let Some(value) = &self.on_beforetoggle {
            write!(f, " {}='{value}'", "on_beforetoggle")?;
        }
        if let Some(value) = &self.on_blur {
            write!(f, " {}='{value}'", "on_blur")?;
        }
        if let Some(value) = &self.on_cancel {
            write!(f, " {}='{value}'", "on_cancel")?;
        }
        if let Some(value) = &self.on_canplay {
            write!(f, " {}='{value}'", "on_canplay")?;
        }
        if let Some(value) = &self.on_canplaythrough {
            write!(f, " {}='{value}'", "on_canplaythrough")?;
        }
        if let Some(value) = &self.on_change {
            write!(f, " {}='{value}'", "on_change")?;
        }
        if let Some(value) = &self.on_click {
            write!(f, " {}='{value}'", "on_click")?;
        }
        if let Some(value) = &self.on_close {
            write!(f, " {}='{value}'", "on_close")?;
        }
        if let Some(value) = &self.on_contextlost {
            write!(f, " {}='{value}'", "on_contextlost")?;
        }
        if let Some(value) = &self.on_contextmenu {
            write!(f, " {}='{value}'", "on_contextmenu")?;
        }
        if let Some(value) = &self.on_contextrestored {
            write!(f, " {}='{value}'", "on_contextrestored")?;
        }
        if let Some(value) = &self.on_copy {
            write!(f, " {}='{value}'", "on_copy")?;
        }
        if let Some(value) = &self.on_cuechange {
            write!(f, " {}='{value}'", "on_cuechange")?;
        }
        if let Some(value) = &self.on_cut {
            write!(f, " {}='{value}'", "on_cut")?;
        }
        if let Some(value) = &self.on_dblclick {
            write!(f, " {}='{value}'", "on_dblclick")?;
        }
        if let Some(value) = &self.on_drag {
            write!(f, " {}='{value}'", "on_drag")?;
        }
        if let Some(value) = &self.on_dragend {
            write!(f, " {}='{value}'", "on_dragend")?;
        }
        if let Some(value) = &self.on_dragenter {
            write!(f, " {}='{value}'", "on_dragenter")?;
        }
        if let Some(value) = &self.on_dragleave {
            write!(f, " {}='{value}'", "on_dragleave")?;
        }
        if let Some(value) = &self.on_dragover {
            write!(f, " {}='{value}'", "on_dragover")?;
        }
        if let Some(value) = &self.on_dragstart {
            write!(f, " {}='{value}'", "on_dragstart")?;
        }
        if let Some(value) = &self.on_drop {
            write!(f, " {}='{value}'", "on_drop")?;
        }
        if let Some(value) = &self.on_durationchange {
            write!(f, " {}='{value}'", "on_durationchange")?;
        }
        if let Some(value) = &self.on_emptied {
            write!(f, " {}='{value}'", "on_emptied")?;
        }
        if let Some(value) = &self.on_ended {
            write!(f, " {}='{value}'", "on_ended")?;
        }
        if let Some(value) = &self.on_error {
            write!(f, " {}='{value}'", "on_error")?;
        }
        if let Some(value) = &self.on_focus {
            write!(f, " {}='{value}'", "on_focus")?;
        }
        if let Some(value) = &self.on_formdata {
            write!(f, " {}='{value}'", "on_formdata")?;
        }
        if let Some(value) = &self.on_input {
            write!(f, " {}='{value}'", "on_input")?;
        }
        if let Some(value) = &self.on_invalid {
            write!(f, " {}='{value}'", "on_invalid")?;
        }
        if let Some(value) = &self.on_keydown {
            write!(f, " {}='{value}'", "on_keydown")?;
        }
        if let Some(value) = &self.on_keypress {
            write!(f, " {}='{value}'", "on_keypress")?;
        }
        if let Some(value) = &self.on_keyup {
            write!(f, " {}='{value}'", "on_keyup")?;
        }
        if let Some(value) = &self.on_load {
            write!(f, " {}='{value}'", "on_load")?;
        }
        if let Some(value) = &self.on_loadeddata {
            write!(f, " {}='{value}'", "on_loadeddata")?;
        }
        if let Some(value) = &self.on_loadedmetadata {
            write!(f, " {}='{value}'", "on_loadedmetadata")?;
        }
        if let Some(value) = &self.on_loadstart {
            write!(f, " {}='{value}'", "on_loadstart")?;
        }
        if let Some(value) = &self.on_mousedown {
            write!(f, " {}='{value}'", "on_mousedown")?;
        }
        if let Some(value) = &self.on_mouseenter {
            write!(f, " {}='{value}'", "on_mouseenter")?;
        }
        if let Some(value) = &self.on_mouseleave {
            write!(f, " {}='{value}'", "on_mouseleave")?;
        }
        if let Some(value) = &self.on_mousemove {
            write!(f, " {}='{value}'", "on_mousemove")?;
        }
        if let Some(value) = &self.on_mouseout {
            write!(f, " {}='{value}'", "on_mouseout")?;
        }
        if let Some(value) = &self.on_mouseover {
            write!(f, " {}='{value}'", "on_mouseover")?;
        }
        if let Some(value) = &self.on_mouseup {
            write!(f, " {}='{value}'", "on_mouseup")?;
        }
        if let Some(value) = &self.on_paste {
            write!(f, " {}='{value}'", "on_paste")?;
        }
        if let Some(value) = &self.on_pause {
            write!(f, " {}='{value}'", "on_pause")?;
        }
        if let Some(value) = &self.on_play {
            write!(f, " {}='{value}'", "on_play")?;
        }
        if let Some(value) = &self.on_playing {
            write!(f, " {}='{value}'", "on_playing")?;
        }
        if let Some(value) = &self.on_progress {
            write!(f, " {}='{value}'", "on_progress")?;
        }
        if let Some(value) = &self.on_ratechange {
            write!(f, " {}='{value}'", "on_ratechange")?;
        }
        if let Some(value) = &self.on_reset {
            write!(f, " {}='{value}'", "on_reset")?;
        }
        if let Some(value) = &self.on_resize {
            write!(f, " {}='{value}'", "on_resize")?;
        }
        if let Some(value) = &self.on_scroll {
            write!(f, " {}='{value}'", "on_scroll")?;
        }
        if let Some(value) = &self.on_scrollend {
            write!(f, " {}='{value}'", "on_scrollend")?;
        }
        if let Some(value) = &self.on_securitypolicyviolation {
            write!(f, " {}='{value}'", "on_securitypolicyviolation")?;
        }
        if let Some(value) = &self.on_seeked {
            write!(f, " {}='{value}'", "on_seeked")?;
        }
        if let Some(value) = &self.on_seeking {
            write!(f, " {}='{value}'", "on_seeking")?;
        }
        if let Some(value) = &self.on_select {
            write!(f, " {}='{value}'", "on_select")?;
        }
        if let Some(value) = &self.on_slotchange {
            write!(f, " {}='{value}'", "on_slotchange")?;
        }
        if let Some(value) = &self.on_stalled {
            write!(f, " {}='{value}'", "on_stalled")?;
        }
        if let Some(value) = &self.on_submit {
            write!(f, " {}='{value}'", "on_submit")?;
        }
        if let Some(value) = &self.on_suspend {
            write!(f, " {}='{value}'", "on_suspend")?;
        }
        if let Some(value) = &self.on_timeupdate {
            write!(f, " {}='{value}'", "on_timeupdate")?;
        }
        if let Some(value) = &self.on_toggle {
            write!(f, " {}='{value}'", "on_toggle")?;
        }
        if let Some(value) = &self.on_volumechange {
            write!(f, " {}='{value}'", "on_volumechange")?;
        }
        if let Some(value) = &self.on_waiting {
            write!(f, " {}='{value}'", "on_waiting")?;
        }
        if let Some(value) = &self.on_wheel {
            write!(f, " {}='{value}'", "on_wheel")?;
        }
        write!(f, ">")?;
        for child in &self.children {
            write!(f, "{child}")?;
        }
        write!(f, "</{}>", self.tag)?;
        Ok(())
    }
}
