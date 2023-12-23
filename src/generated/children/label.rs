// 🤖 This file is generated!

use crate::*;
/// The `<label>` element's children.
#[derive(Clone)]
pub enum LabelChild {
    Abbreviation(Abbreviation),
    Anchor(Anchor),
    Audio(Audio),
    BidirectionalIsolate(BidirectionalIsolate),
    BidirectionalOverride(BidirectionalOverride),
    Bold(Bold),
    Button(Button),
    Canvas(Canvas),
    Cite(Cite),
    Code(Code),
    Custom(Custom),
    Data(Data),
    DataList(DataList),
    Definition(Definition),
    Deleted(Deleted),
    Embed(Embed),
    Emphasis(Emphasis),
    Image(Image),
    InlineFrame(InlineFrame),
    Input(Input),
    Inserted(Inserted),
    Italic(Italic),
    Keyboard(Keyboard),
    Label(Label),
    LineBreak(LineBreak),
    Link(Link),
    Map(Map),
    MapArea(MapArea),
    Mark(Mark),
    Metadata(Metadata),
    Meter(Meter),
    NoScript(NoScript),
    Object(Object),
    Output(Output),
    Picture(Picture),
    Progress(Progress),
    Quote(Quote),
    Ruby(Ruby),
    Sample(Sample),
    Script(Script),
    Select(Select),
    Slot(Slot),
    Small(Small),
    Span(Span),
    StrikeThrough(StrikeThrough),
    Strong(Strong),
    SubScript(SubScript),
    SupScript(SupScript),
    Template(Template),
    TextArea(TextArea),
    Time(Time),
    Underline(Underline),
    Variable(Variable),
    Video(Video),
    WordBreak(WordBreak),
    Text(CowStr),
}
impl From<Abbreviation> for LabelChild {
    fn from(child: Abbreviation) -> Self {
        LabelChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for LabelChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        LabelChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for LabelChild {
    fn from(child: Anchor) -> Self {
        LabelChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for LabelChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        LabelChild::Anchor(builder.build())
    }
}
impl From<Audio> for LabelChild {
    fn from(child: Audio) -> Self {
        LabelChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for LabelChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        LabelChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for LabelChild {
    fn from(child: BidirectionalIsolate) -> Self {
        LabelChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for LabelChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        LabelChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for LabelChild {
    fn from(child: BidirectionalOverride) -> Self {
        LabelChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for LabelChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        LabelChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for LabelChild {
    fn from(child: Bold) -> Self {
        LabelChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for LabelChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        LabelChild::Bold(builder.build())
    }
}
impl From<Button> for LabelChild {
    fn from(child: Button) -> Self {
        LabelChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for LabelChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        LabelChild::Button(builder.build())
    }
}
impl From<Canvas> for LabelChild {
    fn from(child: Canvas) -> Self {
        LabelChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for LabelChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        LabelChild::Canvas(builder.build())
    }
}
impl From<Cite> for LabelChild {
    fn from(child: Cite) -> Self {
        LabelChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for LabelChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        LabelChild::Cite(builder.build())
    }
}
impl From<Code> for LabelChild {
    fn from(child: Code) -> Self {
        LabelChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for LabelChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        LabelChild::Code(builder.build())
    }
}
impl From<Custom> for LabelChild {
    fn from(child: Custom) -> Self {
        LabelChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for LabelChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        LabelChild::Custom(builder.build())
    }
}
impl From<Data> for LabelChild {
    fn from(child: Data) -> Self {
        LabelChild::Data(child)
    }
}
impl From<builders::DataBuilder> for LabelChild {
    fn from(builder: builders::DataBuilder) -> Self {
        LabelChild::Data(builder.build())
    }
}
impl From<DataList> for LabelChild {
    fn from(child: DataList) -> Self {
        LabelChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for LabelChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        LabelChild::DataList(builder.build())
    }
}
impl From<Definition> for LabelChild {
    fn from(child: Definition) -> Self {
        LabelChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for LabelChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        LabelChild::Definition(builder.build())
    }
}
impl From<Deleted> for LabelChild {
    fn from(child: Deleted) -> Self {
        LabelChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for LabelChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        LabelChild::Deleted(builder.build())
    }
}
impl From<Embed> for LabelChild {
    fn from(child: Embed) -> Self {
        LabelChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for LabelChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        LabelChild::Embed(builder.build())
    }
}
impl From<Emphasis> for LabelChild {
    fn from(child: Emphasis) -> Self {
        LabelChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for LabelChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        LabelChild::Emphasis(builder.build())
    }
}
impl From<Image> for LabelChild {
    fn from(child: Image) -> Self {
        LabelChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for LabelChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        LabelChild::Image(builder.build())
    }
}
impl From<InlineFrame> for LabelChild {
    fn from(child: InlineFrame) -> Self {
        LabelChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for LabelChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        LabelChild::InlineFrame(builder.build())
    }
}
impl From<Input> for LabelChild {
    fn from(child: Input) -> Self {
        LabelChild::Input(child)
    }
}
impl From<builders::InputBuilder> for LabelChild {
    fn from(builder: builders::InputBuilder) -> Self {
        LabelChild::Input(builder.build())
    }
}
impl From<Inserted> for LabelChild {
    fn from(child: Inserted) -> Self {
        LabelChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for LabelChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        LabelChild::Inserted(builder.build())
    }
}
impl From<Italic> for LabelChild {
    fn from(child: Italic) -> Self {
        LabelChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for LabelChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        LabelChild::Italic(builder.build())
    }
}
impl From<Keyboard> for LabelChild {
    fn from(child: Keyboard) -> Self {
        LabelChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for LabelChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        LabelChild::Keyboard(builder.build())
    }
}
impl From<Label> for LabelChild {
    fn from(child: Label) -> Self {
        LabelChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for LabelChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        LabelChild::Label(builder.build())
    }
}
impl From<LineBreak> for LabelChild {
    fn from(child: LineBreak) -> Self {
        LabelChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for LabelChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        LabelChild::LineBreak(builder.build())
    }
}
impl From<Link> for LabelChild {
    fn from(child: Link) -> Self {
        LabelChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for LabelChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        LabelChild::Link(builder.build())
    }
}
impl From<Map> for LabelChild {
    fn from(child: Map) -> Self {
        LabelChild::Map(child)
    }
}
impl From<builders::MapBuilder> for LabelChild {
    fn from(builder: builders::MapBuilder) -> Self {
        LabelChild::Map(builder.build())
    }
}
impl From<MapArea> for LabelChild {
    fn from(child: MapArea) -> Self {
        LabelChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for LabelChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        LabelChild::MapArea(builder.build())
    }
}
impl From<Mark> for LabelChild {
    fn from(child: Mark) -> Self {
        LabelChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for LabelChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        LabelChild::Mark(builder.build())
    }
}
impl From<Metadata> for LabelChild {
    fn from(child: Metadata) -> Self {
        LabelChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for LabelChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        LabelChild::Metadata(builder.build())
    }
}
impl From<Meter> for LabelChild {
    fn from(child: Meter) -> Self {
        LabelChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for LabelChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        LabelChild::Meter(builder.build())
    }
}
impl From<NoScript> for LabelChild {
    fn from(child: NoScript) -> Self {
        LabelChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for LabelChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        LabelChild::NoScript(builder.build())
    }
}
impl From<Object> for LabelChild {
    fn from(child: Object) -> Self {
        LabelChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for LabelChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        LabelChild::Object(builder.build())
    }
}
impl From<Output> for LabelChild {
    fn from(child: Output) -> Self {
        LabelChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for LabelChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        LabelChild::Output(builder.build())
    }
}
impl From<Picture> for LabelChild {
    fn from(child: Picture) -> Self {
        LabelChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for LabelChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        LabelChild::Picture(builder.build())
    }
}
impl From<Progress> for LabelChild {
    fn from(child: Progress) -> Self {
        LabelChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for LabelChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        LabelChild::Progress(builder.build())
    }
}
impl From<Quote> for LabelChild {
    fn from(child: Quote) -> Self {
        LabelChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for LabelChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        LabelChild::Quote(builder.build())
    }
}
impl From<Ruby> for LabelChild {
    fn from(child: Ruby) -> Self {
        LabelChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for LabelChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        LabelChild::Ruby(builder.build())
    }
}
impl From<Sample> for LabelChild {
    fn from(child: Sample) -> Self {
        LabelChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for LabelChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        LabelChild::Sample(builder.build())
    }
}
impl From<Script> for LabelChild {
    fn from(child: Script) -> Self {
        LabelChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for LabelChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        LabelChild::Script(builder.build())
    }
}
impl From<Select> for LabelChild {
    fn from(child: Select) -> Self {
        LabelChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for LabelChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        LabelChild::Select(builder.build())
    }
}
impl From<Slot> for LabelChild {
    fn from(child: Slot) -> Self {
        LabelChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for LabelChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        LabelChild::Slot(builder.build())
    }
}
impl From<Small> for LabelChild {
    fn from(child: Small) -> Self {
        LabelChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for LabelChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        LabelChild::Small(builder.build())
    }
}
impl From<Span> for LabelChild {
    fn from(child: Span) -> Self {
        LabelChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for LabelChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        LabelChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for LabelChild {
    fn from(child: StrikeThrough) -> Self {
        LabelChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for LabelChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        LabelChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for LabelChild {
    fn from(child: Strong) -> Self {
        LabelChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for LabelChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        LabelChild::Strong(builder.build())
    }
}
impl From<SubScript> for LabelChild {
    fn from(child: SubScript) -> Self {
        LabelChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for LabelChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        LabelChild::SubScript(builder.build())
    }
}
impl From<SupScript> for LabelChild {
    fn from(child: SupScript) -> Self {
        LabelChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for LabelChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        LabelChild::SupScript(builder.build())
    }
}
impl From<Template> for LabelChild {
    fn from(child: Template) -> Self {
        LabelChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for LabelChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        LabelChild::Template(builder.build())
    }
}
impl From<TextArea> for LabelChild {
    fn from(child: TextArea) -> Self {
        LabelChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for LabelChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        LabelChild::TextArea(builder.build())
    }
}
impl From<Time> for LabelChild {
    fn from(child: Time) -> Self {
        LabelChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for LabelChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        LabelChild::Time(builder.build())
    }
}
impl From<Underline> for LabelChild {
    fn from(child: Underline) -> Self {
        LabelChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for LabelChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        LabelChild::Underline(builder.build())
    }
}
impl From<Variable> for LabelChild {
    fn from(child: Variable) -> Self {
        LabelChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for LabelChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        LabelChild::Variable(builder.build())
    }
}
impl From<Video> for LabelChild {
    fn from(child: Video) -> Self {
        LabelChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for LabelChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        LabelChild::Video(builder.build())
    }
}
impl From<WordBreak> for LabelChild {
    fn from(child: WordBreak) -> Self {
        LabelChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for LabelChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        LabelChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for LabelChild {
    fn from(s: &'static str) -> Self {
        LabelChild::Text(s.into())
    }
}
impl From<String> for LabelChild {
    fn from(s: String) -> Self {
        LabelChild::Text(s.into())
    }
}
impl From<CowStr> for LabelChild {
    fn from(s: CowStr) -> Self {
        LabelChild::Text(s)
    }
}
impl std::fmt::Debug for LabelChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LabelChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Button(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Code(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Data(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Image(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Input(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Label(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Link(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Map(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Object(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Output(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Script(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Select(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Small(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Span(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Template(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Time(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Video(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            LabelChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for LabelChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LabelChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Audio(child) => std::fmt::Display::fmt(child, f),
            LabelChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            LabelChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Bold(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Button(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Cite(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Code(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Custom(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Data(child) => std::fmt::Display::fmt(child, f),
            LabelChild::DataList(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Definition(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Embed(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Image(child) => std::fmt::Display::fmt(child, f),
            LabelChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Input(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Italic(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Label(child) => std::fmt::Display::fmt(child, f),
            LabelChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Link(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Map(child) => std::fmt::Display::fmt(child, f),
            LabelChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Mark(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Meter(child) => std::fmt::Display::fmt(child, f),
            LabelChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Object(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Output(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Picture(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Progress(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Quote(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Sample(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Script(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Select(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Slot(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Small(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Span(child) => std::fmt::Display::fmt(child, f),
            LabelChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Strong(child) => std::fmt::Display::fmt(child, f),
            LabelChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            LabelChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Template(child) => std::fmt::Display::fmt(child, f),
            LabelChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Time(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Underline(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Variable(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Video(child) => std::fmt::Display::fmt(child, f),
            LabelChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            LabelChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
