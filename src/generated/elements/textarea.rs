// 🤖 This file is generated!

use crate::*;
/// The `<textarea>` element.
///
/// [`MDN`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
#[doc(alias = "textarea")]
#[derive(Clone, Default)]
pub struct TextArea {
    pub id: StdOption<CowStr>,
    pub classes: HashSet<CowStr>,
    pub datas: HashMap<CowStr, AttributeType>,
    pub children: Vec<children::TextAreaChild>,
    pub accesskey: std::option::Option<CowStr>,
    pub autocapitalize: std::option::Option<CowStr>,
    pub autocomplete: std::option::Option<CowStr>,
    pub autofocus: std::option::Option<CowStr>,
    pub cols: std::option::Option<u32>,
    pub contenteditable: std::option::Option<CowStr>,
    pub dir: std::option::Option<CowStr>,
    pub dirname: std::option::Option<CowStr>,
    pub disabled: bool,
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
    pub maxlength: std::option::Option<i32>,
    pub minlength: std::option::Option<i32>,
    pub name: std::option::Option<CowStr>,
    pub nonce: std::option::Option<CowStr>,
    pub placeholder: std::option::Option<CowStr>,
    pub popover: std::option::Option<CowStr>,
    pub readonly: bool,
    pub required: bool,
    pub rows: std::option::Option<u32>,
    pub slot: std::option::Option<CowStr>,
    pub spellcheck: bool,
    pub style: std::option::Option<CowStr>,
    pub tabindex: std::option::Option<CowStr>,
    pub title: std::option::Option<CowStr>,
    pub translate: bool,
    pub wrap: std::option::Option<CowStr>,
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
}
impl TextArea {
    pub fn new() -> Self {
        <Self as Default>::default()
    }
    pub fn id<T: Into<CowStr>>(id: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().id(id)
    }
    pub fn class<T: Into<CowStr>>(class: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().class(class)
    }
    pub fn classes<T: Into<CowStr>, I: IntoIterator<Item = T>>(
        classes: I,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().classes(classes)
    }
    pub fn data<K: Into<CowStr>, V: Into<AttributeType>>(
        key: K,
        value: V,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().data(key, value)
    }
    pub fn datas<
        K: Into<CowStr>,
        V: Into<AttributeType>,
        I: IntoIterator<Item = (K, V)>,
    >(datas: I) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().datas(datas)
    }
    pub fn child<T: Into<children::TextAreaChild>>(
        child: T,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().child(child)
    }
    pub fn children<T: Into<children::TextAreaChild>, I: IntoIterator<Item = T>>(
        children: I,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().children(children)
    }
    pub fn accesskey<T: Into<CowStr>>(accesskey: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().accesskey(accesskey)
    }
    pub fn autocapitalize<T: Into<CowStr>>(
        autocapitalize: T,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().autocapitalize(autocapitalize)
    }
    pub fn autocomplete<T: Into<CowStr>>(autocomplete: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().autocomplete(autocomplete)
    }
    pub fn autofocus<T: Into<CowStr>>(autofocus: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().autofocus(autofocus)
    }
    pub fn cols<T: Into<u32>>(cols: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().cols(cols)
    }
    pub fn contenteditable<T: Into<CowStr>>(
        contenteditable: T,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default()
            .contenteditable(contenteditable)
    }
    pub fn dir<T: Into<CowStr>>(dir: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().dir(dir)
    }
    pub fn dirname<T: Into<CowStr>>(dirname: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().dirname(dirname)
    }
    pub fn disabled<T: Into<bool>>(disabled: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().disabled(disabled)
    }
    pub fn draggable<T: Into<bool>>(draggable: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().draggable(draggable)
    }
    pub fn enterkeyhint<T: Into<CowStr>>(enterkeyhint: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().enterkeyhint(enterkeyhint)
    }
    pub fn form<T: Into<CowStr>>(form: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().form(form)
    }
    pub fn hidden<T: Into<BoolOrF64OrString>>(hidden: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().hidden(hidden)
    }
    pub fn inert<T: Into<bool>>(inert: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().inert(inert)
    }
    pub fn inputmode<T: Into<CowStr>>(inputmode: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().inputmode(inputmode)
    }
    pub fn is<T: Into<CowStr>>(is: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().is(is)
    }
    pub fn itemid<T: Into<CowStr>>(itemid: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().itemid(itemid)
    }
    pub fn itemprop<T: Into<CowStr>>(itemprop: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().itemprop(itemprop)
    }
    pub fn itemref<T: Into<CowStr>>(itemref: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().itemref(itemref)
    }
    pub fn itemscope<T: Into<CowStr>>(itemscope: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().itemscope(itemscope)
    }
    pub fn itemtype<T: Into<CowStr>>(itemtype: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().itemtype(itemtype)
    }
    pub fn lang<T: Into<CowStr>>(lang: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().lang(lang)
    }
    pub fn maxlength<T: Into<i32>>(maxlength: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().maxlength(maxlength)
    }
    pub fn minlength<T: Into<i32>>(minlength: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().minlength(minlength)
    }
    pub fn name<T: Into<CowStr>>(name: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().name(name)
    }
    pub fn nonce<T: Into<CowStr>>(nonce: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().nonce(nonce)
    }
    pub fn placeholder<T: Into<CowStr>>(placeholder: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().placeholder(placeholder)
    }
    pub fn popover<T: Into<CowStr>>(popover: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().popover(popover)
    }
    pub fn readonly<T: Into<bool>>(readonly: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().readonly(readonly)
    }
    pub fn required<T: Into<bool>>(required: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().required(required)
    }
    pub fn rows<T: Into<u32>>(rows: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().rows(rows)
    }
    pub fn slot<T: Into<CowStr>>(slot: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().slot(slot)
    }
    pub fn spellcheck<T: Into<bool>>(spellcheck: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().spellcheck(spellcheck)
    }
    pub fn style<T: Into<CowStr>>(style: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().style(style)
    }
    pub fn tabindex<T: Into<CowStr>>(tabindex: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().tabindex(tabindex)
    }
    pub fn title<T: Into<CowStr>>(title: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().title(title)
    }
    pub fn translate<T: Into<bool>>(translate: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().translate(translate)
    }
    pub fn wrap<T: Into<CowStr>>(wrap: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().wrap(wrap)
    }
    pub fn on_auxclick<T: Into<CowStr>>(on_auxclick: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_auxclick(on_auxclick)
    }
    pub fn on_beforeinput<T: Into<CowStr>>(
        on_beforeinput: T,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_beforeinput(on_beforeinput)
    }
    pub fn on_beforematch<T: Into<CowStr>>(
        on_beforematch: T,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_beforematch(on_beforematch)
    }
    pub fn on_beforetoggle<T: Into<CowStr>>(
        on_beforetoggle: T,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default()
            .on_beforetoggle(on_beforetoggle)
    }
    pub fn on_blur<T: Into<CowStr>>(on_blur: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_blur(on_blur)
    }
    pub fn on_cancel<T: Into<CowStr>>(on_cancel: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_cancel(on_cancel)
    }
    pub fn on_canplay<T: Into<CowStr>>(on_canplay: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_canplay(on_canplay)
    }
    pub fn on_canplaythrough<T: Into<CowStr>>(
        on_canplaythrough: T,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default()
            .on_canplaythrough(on_canplaythrough)
    }
    pub fn on_change<T: Into<CowStr>>(on_change: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_change(on_change)
    }
    pub fn on_click<T: Into<CowStr>>(on_click: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_click(on_click)
    }
    pub fn on_close<T: Into<CowStr>>(on_close: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_close(on_close)
    }
    pub fn on_contextlost<T: Into<CowStr>>(
        on_contextlost: T,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_contextlost(on_contextlost)
    }
    pub fn on_contextmenu<T: Into<CowStr>>(
        on_contextmenu: T,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_contextmenu(on_contextmenu)
    }
    pub fn on_contextrestored<T: Into<CowStr>>(
        on_contextrestored: T,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default()
            .on_contextrestored(on_contextrestored)
    }
    pub fn on_copy<T: Into<CowStr>>(on_copy: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_copy(on_copy)
    }
    pub fn on_cuechange<T: Into<CowStr>>(on_cuechange: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_cuechange(on_cuechange)
    }
    pub fn on_cut<T: Into<CowStr>>(on_cut: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_cut(on_cut)
    }
    pub fn on_dblclick<T: Into<CowStr>>(on_dblclick: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_dblclick(on_dblclick)
    }
    pub fn on_drag<T: Into<CowStr>>(on_drag: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_drag(on_drag)
    }
    pub fn on_dragend<T: Into<CowStr>>(on_dragend: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_dragend(on_dragend)
    }
    pub fn on_dragenter<T: Into<CowStr>>(on_dragenter: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_dragenter(on_dragenter)
    }
    pub fn on_dragleave<T: Into<CowStr>>(on_dragleave: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_dragleave(on_dragleave)
    }
    pub fn on_dragover<T: Into<CowStr>>(on_dragover: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_dragover(on_dragover)
    }
    pub fn on_dragstart<T: Into<CowStr>>(on_dragstart: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_dragstart(on_dragstart)
    }
    pub fn on_drop<T: Into<CowStr>>(on_drop: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_drop(on_drop)
    }
    pub fn on_durationchange<T: Into<CowStr>>(
        on_durationchange: T,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default()
            .on_durationchange(on_durationchange)
    }
    pub fn on_emptied<T: Into<CowStr>>(on_emptied: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_emptied(on_emptied)
    }
    pub fn on_ended<T: Into<CowStr>>(on_ended: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_ended(on_ended)
    }
    pub fn on_error<T: Into<CowStr>>(on_error: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_error(on_error)
    }
    pub fn on_focus<T: Into<CowStr>>(on_focus: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_focus(on_focus)
    }
    pub fn on_formdata<T: Into<CowStr>>(on_formdata: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_formdata(on_formdata)
    }
    pub fn on_input<T: Into<CowStr>>(on_input: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_input(on_input)
    }
    pub fn on_invalid<T: Into<CowStr>>(on_invalid: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_invalid(on_invalid)
    }
    pub fn on_keydown<T: Into<CowStr>>(on_keydown: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_keydown(on_keydown)
    }
    pub fn on_keypress<T: Into<CowStr>>(on_keypress: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_keypress(on_keypress)
    }
    pub fn on_keyup<T: Into<CowStr>>(on_keyup: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_keyup(on_keyup)
    }
    pub fn on_load<T: Into<CowStr>>(on_load: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_load(on_load)
    }
    pub fn on_loadeddata<T: Into<CowStr>>(
        on_loadeddata: T,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_loadeddata(on_loadeddata)
    }
    pub fn on_loadedmetadata<T: Into<CowStr>>(
        on_loadedmetadata: T,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default()
            .on_loadedmetadata(on_loadedmetadata)
    }
    pub fn on_loadstart<T: Into<CowStr>>(on_loadstart: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_loadstart(on_loadstart)
    }
    pub fn on_mousedown<T: Into<CowStr>>(on_mousedown: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_mousedown(on_mousedown)
    }
    pub fn on_mouseenter<T: Into<CowStr>>(
        on_mouseenter: T,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_mouseenter(on_mouseenter)
    }
    pub fn on_mouseleave<T: Into<CowStr>>(
        on_mouseleave: T,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_mouseleave(on_mouseleave)
    }
    pub fn on_mousemove<T: Into<CowStr>>(on_mousemove: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_mousemove(on_mousemove)
    }
    pub fn on_mouseout<T: Into<CowStr>>(on_mouseout: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_mouseout(on_mouseout)
    }
    pub fn on_mouseover<T: Into<CowStr>>(on_mouseover: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_mouseover(on_mouseover)
    }
    pub fn on_mouseup<T: Into<CowStr>>(on_mouseup: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_mouseup(on_mouseup)
    }
    pub fn on_paste<T: Into<CowStr>>(on_paste: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_paste(on_paste)
    }
    pub fn on_pause<T: Into<CowStr>>(on_pause: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_pause(on_pause)
    }
    pub fn on_play<T: Into<CowStr>>(on_play: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_play(on_play)
    }
    pub fn on_playing<T: Into<CowStr>>(on_playing: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_playing(on_playing)
    }
    pub fn on_progress<T: Into<CowStr>>(on_progress: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_progress(on_progress)
    }
    pub fn on_ratechange<T: Into<CowStr>>(
        on_ratechange: T,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_ratechange(on_ratechange)
    }
    pub fn on_reset<T: Into<CowStr>>(on_reset: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_reset(on_reset)
    }
    pub fn on_resize<T: Into<CowStr>>(on_resize: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_resize(on_resize)
    }
    pub fn on_scroll<T: Into<CowStr>>(on_scroll: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_scroll(on_scroll)
    }
    pub fn on_scrollend<T: Into<CowStr>>(on_scrollend: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_scrollend(on_scrollend)
    }
    pub fn on_securitypolicyviolation<T: Into<CowStr>>(
        on_securitypolicyviolation: T,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default()
            .on_securitypolicyviolation(on_securitypolicyviolation)
    }
    pub fn on_seeked<T: Into<CowStr>>(on_seeked: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_seeked(on_seeked)
    }
    pub fn on_seeking<T: Into<CowStr>>(on_seeking: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_seeking(on_seeking)
    }
    pub fn on_select<T: Into<CowStr>>(on_select: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_select(on_select)
    }
    pub fn on_slotchange<T: Into<CowStr>>(
        on_slotchange: T,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_slotchange(on_slotchange)
    }
    pub fn on_stalled<T: Into<CowStr>>(on_stalled: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_stalled(on_stalled)
    }
    pub fn on_submit<T: Into<CowStr>>(on_submit: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_submit(on_submit)
    }
    pub fn on_suspend<T: Into<CowStr>>(on_suspend: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_suspend(on_suspend)
    }
    pub fn on_timeupdate<T: Into<CowStr>>(
        on_timeupdate: T,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_timeupdate(on_timeupdate)
    }
    pub fn on_toggle<T: Into<CowStr>>(on_toggle: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_toggle(on_toggle)
    }
    pub fn on_volumechange<T: Into<CowStr>>(
        on_volumechange: T,
    ) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default()
            .on_volumechange(on_volumechange)
    }
    pub fn on_waiting<T: Into<CowStr>>(on_waiting: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_waiting(on_waiting)
    }
    pub fn on_wheel<T: Into<CowStr>>(on_wheel: T) -> builders::TextAreaBuilder {
        <builders::TextAreaBuilder as Default>::default().on_wheel(on_wheel)
    }
}
impl From<builders::TextAreaBuilder> for TextArea {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        builder.element
    }
}
impl std::fmt::Debug for TextArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct(&format!("<{}>", "textarea"));
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
        if let Some(value) = &self.autocomplete {
            f.field("autocomplete", &value);
        }
        if let Some(value) = &self.autofocus {
            f.field("autofocus", &value);
        }
        if let Some(value) = &self.cols {
            f.field("cols", &value);
        }
        if let Some(value) = &self.contenteditable {
            f.field("contenteditable", &value);
        }
        if let Some(value) = &self.dir {
            f.field("dir", &value);
        }
        if let Some(value) = &self.dirname {
            f.field("dirname", &value);
        }
        if self.disabled {
            f.field("disabled", &true);
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
        if let Some(value) = &self.maxlength {
            f.field("maxlength", &value);
        }
        if let Some(value) = &self.minlength {
            f.field("minlength", &value);
        }
        if let Some(value) = &self.name {
            f.field("name", &value);
        }
        if let Some(value) = &self.nonce {
            f.field("nonce", &value);
        }
        if let Some(value) = &self.placeholder {
            f.field("placeholder", &value);
        }
        if let Some(value) = &self.popover {
            f.field("popover", &value);
        }
        if self.readonly {
            f.field("readonly", &true);
        }
        if self.required {
            f.field("required", &true);
        }
        if let Some(value) = &self.rows {
            f.field("rows", &value);
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
        if let Some(value) = &self.wrap {
            f.field("wrap", &value);
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
impl std::fmt::Display for TextArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}", "textarea")?;
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
        if let Some(value) = &self.autocomplete {
            write!(f, " {}='{value}'", "autocomplete")?;
        }
        if let Some(value) = &self.autofocus {
            write!(f, " {}='{value}'", "autofocus")?;
        }
        if let Some(value) = &self.cols {
            write!(f, " {}={value}", "cols")?;
        }
        if let Some(value) = &self.contenteditable {
            write!(f, " {}='{value}'", "contenteditable")?;
        }
        if let Some(value) = &self.dir {
            write!(f, " {}='{value}'", "dir")?;
        }
        if let Some(value) = &self.dirname {
            write!(f, " {}='{value}'", "dirname")?;
        }
        if self.disabled {
            write!(f, " {}", "disabled")?;
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
        if let Some(value) = &self.maxlength {
            write!(f, " {}={value}", "maxlength")?;
        }
        if let Some(value) = &self.minlength {
            write!(f, " {}={value}", "minlength")?;
        }
        if let Some(value) = &self.name {
            write!(f, " {}='{value}'", "name")?;
        }
        if let Some(value) = &self.nonce {
            write!(f, " {}='{value}'", "nonce")?;
        }
        if let Some(value) = &self.placeholder {
            write!(f, " {}='{value}'", "placeholder")?;
        }
        if let Some(value) = &self.popover {
            write!(f, " {}='{value}'", "popover")?;
        }
        if self.readonly {
            write!(f, " {}", "readonly")?;
        }
        if self.required {
            write!(f, " {}", "required")?;
        }
        if let Some(value) = &self.rows {
            write!(f, " {}={value}", "rows")?;
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
            write!(f, " {}='{value}'", "tabindex")?;
        }
        if let Some(value) = &self.title {
            write!(f, " {}='{value}'", "title")?;
        }
        if self.translate {
            write!(f, " {}", "translate")?;
        }
        if let Some(value) = &self.wrap {
            write!(f, " {}='{value}'", "wrap")?;
        }
        if let Some(value) = &self.on_auxclick {
            write!(f, " {}='{value}'", "onauxclick")?;
        }
        if let Some(value) = &self.on_beforeinput {
            write!(f, " {}='{value}'", "onbeforeinput")?;
        }
        if let Some(value) = &self.on_beforematch {
            write!(f, " {}='{value}'", "onbeforematch")?;
        }
        if let Some(value) = &self.on_beforetoggle {
            write!(f, " {}='{value}'", "onbeforetoggle")?;
        }
        if let Some(value) = &self.on_blur {
            write!(f, " {}='{value}'", "onblur")?;
        }
        if let Some(value) = &self.on_cancel {
            write!(f, " {}='{value}'", "oncancel")?;
        }
        if let Some(value) = &self.on_canplay {
            write!(f, " {}='{value}'", "oncanplay")?;
        }
        if let Some(value) = &self.on_canplaythrough {
            write!(f, " {}='{value}'", "oncanplaythrough")?;
        }
        if let Some(value) = &self.on_change {
            write!(f, " {}='{value}'", "onchange")?;
        }
        if let Some(value) = &self.on_click {
            write!(f, " {}='{value}'", "onclick")?;
        }
        if let Some(value) = &self.on_close {
            write!(f, " {}='{value}'", "onclose")?;
        }
        if let Some(value) = &self.on_contextlost {
            write!(f, " {}='{value}'", "oncontextlost")?;
        }
        if let Some(value) = &self.on_contextmenu {
            write!(f, " {}='{value}'", "oncontextmenu")?;
        }
        if let Some(value) = &self.on_contextrestored {
            write!(f, " {}='{value}'", "oncontextrestored")?;
        }
        if let Some(value) = &self.on_copy {
            write!(f, " {}='{value}'", "oncopy")?;
        }
        if let Some(value) = &self.on_cuechange {
            write!(f, " {}='{value}'", "oncuechange")?;
        }
        if let Some(value) = &self.on_cut {
            write!(f, " {}='{value}'", "oncut")?;
        }
        if let Some(value) = &self.on_dblclick {
            write!(f, " {}='{value}'", "ondblclick")?;
        }
        if let Some(value) = &self.on_drag {
            write!(f, " {}='{value}'", "ondrag")?;
        }
        if let Some(value) = &self.on_dragend {
            write!(f, " {}='{value}'", "ondragend")?;
        }
        if let Some(value) = &self.on_dragenter {
            write!(f, " {}='{value}'", "ondragenter")?;
        }
        if let Some(value) = &self.on_dragleave {
            write!(f, " {}='{value}'", "ondragleave")?;
        }
        if let Some(value) = &self.on_dragover {
            write!(f, " {}='{value}'", "ondragover")?;
        }
        if let Some(value) = &self.on_dragstart {
            write!(f, " {}='{value}'", "ondragstart")?;
        }
        if let Some(value) = &self.on_drop {
            write!(f, " {}='{value}'", "ondrop")?;
        }
        if let Some(value) = &self.on_durationchange {
            write!(f, " {}='{value}'", "ondurationchange")?;
        }
        if let Some(value) = &self.on_emptied {
            write!(f, " {}='{value}'", "onemptied")?;
        }
        if let Some(value) = &self.on_ended {
            write!(f, " {}='{value}'", "onended")?;
        }
        if let Some(value) = &self.on_error {
            write!(f, " {}='{value}'", "onerror")?;
        }
        if let Some(value) = &self.on_focus {
            write!(f, " {}='{value}'", "onfocus")?;
        }
        if let Some(value) = &self.on_formdata {
            write!(f, " {}='{value}'", "onformdata")?;
        }
        if let Some(value) = &self.on_input {
            write!(f, " {}='{value}'", "oninput")?;
        }
        if let Some(value) = &self.on_invalid {
            write!(f, " {}='{value}'", "oninvalid")?;
        }
        if let Some(value) = &self.on_keydown {
            write!(f, " {}='{value}'", "onkeydown")?;
        }
        if let Some(value) = &self.on_keypress {
            write!(f, " {}='{value}'", "onkeypress")?;
        }
        if let Some(value) = &self.on_keyup {
            write!(f, " {}='{value}'", "onkeyup")?;
        }
        if let Some(value) = &self.on_load {
            write!(f, " {}='{value}'", "onload")?;
        }
        if let Some(value) = &self.on_loadeddata {
            write!(f, " {}='{value}'", "onloadeddata")?;
        }
        if let Some(value) = &self.on_loadedmetadata {
            write!(f, " {}='{value}'", "onloadedmetadata")?;
        }
        if let Some(value) = &self.on_loadstart {
            write!(f, " {}='{value}'", "onloadstart")?;
        }
        if let Some(value) = &self.on_mousedown {
            write!(f, " {}='{value}'", "onmousedown")?;
        }
        if let Some(value) = &self.on_mouseenter {
            write!(f, " {}='{value}'", "onmouseenter")?;
        }
        if let Some(value) = &self.on_mouseleave {
            write!(f, " {}='{value}'", "onmouseleave")?;
        }
        if let Some(value) = &self.on_mousemove {
            write!(f, " {}='{value}'", "onmousemove")?;
        }
        if let Some(value) = &self.on_mouseout {
            write!(f, " {}='{value}'", "onmouseout")?;
        }
        if let Some(value) = &self.on_mouseover {
            write!(f, " {}='{value}'", "onmouseover")?;
        }
        if let Some(value) = &self.on_mouseup {
            write!(f, " {}='{value}'", "onmouseup")?;
        }
        if let Some(value) = &self.on_paste {
            write!(f, " {}='{value}'", "onpaste")?;
        }
        if let Some(value) = &self.on_pause {
            write!(f, " {}='{value}'", "onpause")?;
        }
        if let Some(value) = &self.on_play {
            write!(f, " {}='{value}'", "onplay")?;
        }
        if let Some(value) = &self.on_playing {
            write!(f, " {}='{value}'", "onplaying")?;
        }
        if let Some(value) = &self.on_progress {
            write!(f, " {}='{value}'", "onprogress")?;
        }
        if let Some(value) = &self.on_ratechange {
            write!(f, " {}='{value}'", "onratechange")?;
        }
        if let Some(value) = &self.on_reset {
            write!(f, " {}='{value}'", "onreset")?;
        }
        if let Some(value) = &self.on_resize {
            write!(f, " {}='{value}'", "onresize")?;
        }
        if let Some(value) = &self.on_scroll {
            write!(f, " {}='{value}'", "onscroll")?;
        }
        if let Some(value) = &self.on_scrollend {
            write!(f, " {}='{value}'", "onscrollend")?;
        }
        if let Some(value) = &self.on_securitypolicyviolation {
            write!(f, " {}='{value}'", "onsecuritypolicyviolation")?;
        }
        if let Some(value) = &self.on_seeked {
            write!(f, " {}='{value}'", "onseeked")?;
        }
        if let Some(value) = &self.on_seeking {
            write!(f, " {}='{value}'", "onseeking")?;
        }
        if let Some(value) = &self.on_select {
            write!(f, " {}='{value}'", "onselect")?;
        }
        if let Some(value) = &self.on_slotchange {
            write!(f, " {}='{value}'", "onslotchange")?;
        }
        if let Some(value) = &self.on_stalled {
            write!(f, " {}='{value}'", "onstalled")?;
        }
        if let Some(value) = &self.on_submit {
            write!(f, " {}='{value}'", "onsubmit")?;
        }
        if let Some(value) = &self.on_suspend {
            write!(f, " {}='{value}'", "onsuspend")?;
        }
        if let Some(value) = &self.on_timeupdate {
            write!(f, " {}='{value}'", "ontimeupdate")?;
        }
        if let Some(value) = &self.on_toggle {
            write!(f, " {}='{value}'", "ontoggle")?;
        }
        if let Some(value) = &self.on_volumechange {
            write!(f, " {}='{value}'", "onvolumechange")?;
        }
        if let Some(value) = &self.on_waiting {
            write!(f, " {}='{value}'", "onwaiting")?;
        }
        if let Some(value) = &self.on_wheel {
            write!(f, " {}='{value}'", "onwheel")?;
        }
        write!(f, ">")?;
        for child in &self.children {
            write!(f, "{child}")?;
        }
        write!(f, "</{}>", "textarea")?;
        Ok(())
    }
}
