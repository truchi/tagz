// 🤖 This file is generated!

use crate::*;
/// The `<span>` element's children.
#[derive(Clone)]
pub enum SpanChild {
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
impl From<Abbreviation> for SpanChild {
    fn from(child: Abbreviation) -> Self {
        SpanChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for SpanChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        SpanChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for SpanChild {
    fn from(child: Anchor) -> Self {
        SpanChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for SpanChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        SpanChild::Anchor(builder.build())
    }
}
impl From<Audio> for SpanChild {
    fn from(child: Audio) -> Self {
        SpanChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for SpanChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        SpanChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for SpanChild {
    fn from(child: BidirectionalIsolate) -> Self {
        SpanChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for SpanChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        SpanChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for SpanChild {
    fn from(child: BidirectionalOverride) -> Self {
        SpanChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for SpanChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        SpanChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for SpanChild {
    fn from(child: Bold) -> Self {
        SpanChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for SpanChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        SpanChild::Bold(builder.build())
    }
}
impl From<Button> for SpanChild {
    fn from(child: Button) -> Self {
        SpanChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for SpanChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        SpanChild::Button(builder.build())
    }
}
impl From<Canvas> for SpanChild {
    fn from(child: Canvas) -> Self {
        SpanChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for SpanChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        SpanChild::Canvas(builder.build())
    }
}
impl From<Cite> for SpanChild {
    fn from(child: Cite) -> Self {
        SpanChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for SpanChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        SpanChild::Cite(builder.build())
    }
}
impl From<Code> for SpanChild {
    fn from(child: Code) -> Self {
        SpanChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for SpanChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        SpanChild::Code(builder.build())
    }
}
impl From<Custom> for SpanChild {
    fn from(child: Custom) -> Self {
        SpanChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for SpanChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        SpanChild::Custom(builder.build())
    }
}
impl From<Data> for SpanChild {
    fn from(child: Data) -> Self {
        SpanChild::Data(child)
    }
}
impl From<builders::DataBuilder> for SpanChild {
    fn from(builder: builders::DataBuilder) -> Self {
        SpanChild::Data(builder.build())
    }
}
impl From<DataList> for SpanChild {
    fn from(child: DataList) -> Self {
        SpanChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for SpanChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        SpanChild::DataList(builder.build())
    }
}
impl From<Definition> for SpanChild {
    fn from(child: Definition) -> Self {
        SpanChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for SpanChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        SpanChild::Definition(builder.build())
    }
}
impl From<Deleted> for SpanChild {
    fn from(child: Deleted) -> Self {
        SpanChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for SpanChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        SpanChild::Deleted(builder.build())
    }
}
impl From<Embed> for SpanChild {
    fn from(child: Embed) -> Self {
        SpanChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for SpanChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        SpanChild::Embed(builder.build())
    }
}
impl From<Emphasis> for SpanChild {
    fn from(child: Emphasis) -> Self {
        SpanChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for SpanChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        SpanChild::Emphasis(builder.build())
    }
}
impl From<Image> for SpanChild {
    fn from(child: Image) -> Self {
        SpanChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for SpanChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        SpanChild::Image(builder.build())
    }
}
impl From<InlineFrame> for SpanChild {
    fn from(child: InlineFrame) -> Self {
        SpanChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for SpanChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        SpanChild::InlineFrame(builder.build())
    }
}
impl From<Input> for SpanChild {
    fn from(child: Input) -> Self {
        SpanChild::Input(child)
    }
}
impl From<builders::InputBuilder> for SpanChild {
    fn from(builder: builders::InputBuilder) -> Self {
        SpanChild::Input(builder.build())
    }
}
impl From<Inserted> for SpanChild {
    fn from(child: Inserted) -> Self {
        SpanChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for SpanChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        SpanChild::Inserted(builder.build())
    }
}
impl From<Italic> for SpanChild {
    fn from(child: Italic) -> Self {
        SpanChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for SpanChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        SpanChild::Italic(builder.build())
    }
}
impl From<Keyboard> for SpanChild {
    fn from(child: Keyboard) -> Self {
        SpanChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for SpanChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        SpanChild::Keyboard(builder.build())
    }
}
impl From<Label> for SpanChild {
    fn from(child: Label) -> Self {
        SpanChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for SpanChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        SpanChild::Label(builder.build())
    }
}
impl From<LineBreak> for SpanChild {
    fn from(child: LineBreak) -> Self {
        SpanChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for SpanChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        SpanChild::LineBreak(builder.build())
    }
}
impl From<Link> for SpanChild {
    fn from(child: Link) -> Self {
        SpanChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for SpanChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        SpanChild::Link(builder.build())
    }
}
impl From<Map> for SpanChild {
    fn from(child: Map) -> Self {
        SpanChild::Map(child)
    }
}
impl From<builders::MapBuilder> for SpanChild {
    fn from(builder: builders::MapBuilder) -> Self {
        SpanChild::Map(builder.build())
    }
}
impl From<MapArea> for SpanChild {
    fn from(child: MapArea) -> Self {
        SpanChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for SpanChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        SpanChild::MapArea(builder.build())
    }
}
impl From<Mark> for SpanChild {
    fn from(child: Mark) -> Self {
        SpanChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for SpanChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        SpanChild::Mark(builder.build())
    }
}
impl From<Metadata> for SpanChild {
    fn from(child: Metadata) -> Self {
        SpanChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for SpanChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        SpanChild::Metadata(builder.build())
    }
}
impl From<Meter> for SpanChild {
    fn from(child: Meter) -> Self {
        SpanChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for SpanChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        SpanChild::Meter(builder.build())
    }
}
impl From<NoScript> for SpanChild {
    fn from(child: NoScript) -> Self {
        SpanChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for SpanChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        SpanChild::NoScript(builder.build())
    }
}
impl From<Object> for SpanChild {
    fn from(child: Object) -> Self {
        SpanChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for SpanChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        SpanChild::Object(builder.build())
    }
}
impl From<Output> for SpanChild {
    fn from(child: Output) -> Self {
        SpanChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for SpanChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        SpanChild::Output(builder.build())
    }
}
impl From<Picture> for SpanChild {
    fn from(child: Picture) -> Self {
        SpanChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for SpanChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        SpanChild::Picture(builder.build())
    }
}
impl From<Progress> for SpanChild {
    fn from(child: Progress) -> Self {
        SpanChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for SpanChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        SpanChild::Progress(builder.build())
    }
}
impl From<Quote> for SpanChild {
    fn from(child: Quote) -> Self {
        SpanChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for SpanChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        SpanChild::Quote(builder.build())
    }
}
impl From<Ruby> for SpanChild {
    fn from(child: Ruby) -> Self {
        SpanChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for SpanChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        SpanChild::Ruby(builder.build())
    }
}
impl From<Sample> for SpanChild {
    fn from(child: Sample) -> Self {
        SpanChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for SpanChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        SpanChild::Sample(builder.build())
    }
}
impl From<Script> for SpanChild {
    fn from(child: Script) -> Self {
        SpanChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for SpanChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        SpanChild::Script(builder.build())
    }
}
impl From<Select> for SpanChild {
    fn from(child: Select) -> Self {
        SpanChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for SpanChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        SpanChild::Select(builder.build())
    }
}
impl From<Slot> for SpanChild {
    fn from(child: Slot) -> Self {
        SpanChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for SpanChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        SpanChild::Slot(builder.build())
    }
}
impl From<Small> for SpanChild {
    fn from(child: Small) -> Self {
        SpanChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for SpanChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        SpanChild::Small(builder.build())
    }
}
impl From<Span> for SpanChild {
    fn from(child: Span) -> Self {
        SpanChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for SpanChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        SpanChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for SpanChild {
    fn from(child: StrikeThrough) -> Self {
        SpanChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for SpanChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        SpanChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for SpanChild {
    fn from(child: Strong) -> Self {
        SpanChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for SpanChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        SpanChild::Strong(builder.build())
    }
}
impl From<SubScript> for SpanChild {
    fn from(child: SubScript) -> Self {
        SpanChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for SpanChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        SpanChild::SubScript(builder.build())
    }
}
impl From<SupScript> for SpanChild {
    fn from(child: SupScript) -> Self {
        SpanChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for SpanChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        SpanChild::SupScript(builder.build())
    }
}
impl From<Template> for SpanChild {
    fn from(child: Template) -> Self {
        SpanChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for SpanChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        SpanChild::Template(builder.build())
    }
}
impl From<TextArea> for SpanChild {
    fn from(child: TextArea) -> Self {
        SpanChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for SpanChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        SpanChild::TextArea(builder.build())
    }
}
impl From<Time> for SpanChild {
    fn from(child: Time) -> Self {
        SpanChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for SpanChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        SpanChild::Time(builder.build())
    }
}
impl From<Underline> for SpanChild {
    fn from(child: Underline) -> Self {
        SpanChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for SpanChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        SpanChild::Underline(builder.build())
    }
}
impl From<Variable> for SpanChild {
    fn from(child: Variable) -> Self {
        SpanChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for SpanChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        SpanChild::Variable(builder.build())
    }
}
impl From<Video> for SpanChild {
    fn from(child: Video) -> Self {
        SpanChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for SpanChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        SpanChild::Video(builder.build())
    }
}
impl From<WordBreak> for SpanChild {
    fn from(child: WordBreak) -> Self {
        SpanChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for SpanChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        SpanChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for SpanChild {
    fn from(s: &'static str) -> Self {
        SpanChild::Text(s.into())
    }
}
impl From<String> for SpanChild {
    fn from(s: String) -> Self {
        SpanChild::Text(s.into())
    }
}
impl From<CowStr> for SpanChild {
    fn from(s: CowStr) -> Self {
        SpanChild::Text(s)
    }
}
impl std::fmt::Debug for SpanChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpanChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Button(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Code(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Data(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Image(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Input(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Label(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Link(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Map(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Object(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Output(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Script(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Select(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Small(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Span(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Template(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Time(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Video(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            SpanChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for SpanChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpanChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Audio(child) => std::fmt::Display::fmt(child, f),
            SpanChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            SpanChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Bold(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Button(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Cite(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Code(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Custom(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Data(child) => std::fmt::Display::fmt(child, f),
            SpanChild::DataList(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Definition(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Embed(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Image(child) => std::fmt::Display::fmt(child, f),
            SpanChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Input(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Italic(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Label(child) => std::fmt::Display::fmt(child, f),
            SpanChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Link(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Map(child) => std::fmt::Display::fmt(child, f),
            SpanChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Mark(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Meter(child) => std::fmt::Display::fmt(child, f),
            SpanChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Object(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Output(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Picture(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Progress(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Quote(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Sample(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Script(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Select(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Slot(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Small(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Span(child) => std::fmt::Display::fmt(child, f),
            SpanChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Strong(child) => std::fmt::Display::fmt(child, f),
            SpanChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            SpanChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Template(child) => std::fmt::Display::fmt(child, f),
            SpanChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Time(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Underline(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Variable(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Video(child) => std::fmt::Display::fmt(child, f),
            SpanChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            SpanChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
