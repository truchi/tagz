// 🤖 This file is generated!

use crate::*;
/// The `<time>` element's children.
#[derive(Clone)]
pub enum TimeChild {
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
impl From<Abbreviation> for TimeChild {
    fn from(child: Abbreviation) -> Self {
        TimeChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for TimeChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        TimeChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for TimeChild {
    fn from(child: Anchor) -> Self {
        TimeChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for TimeChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        TimeChild::Anchor(builder.build())
    }
}
impl From<Audio> for TimeChild {
    fn from(child: Audio) -> Self {
        TimeChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for TimeChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        TimeChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for TimeChild {
    fn from(child: BidirectionalIsolate) -> Self {
        TimeChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for TimeChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        TimeChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for TimeChild {
    fn from(child: BidirectionalOverride) -> Self {
        TimeChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for TimeChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        TimeChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for TimeChild {
    fn from(child: Bold) -> Self {
        TimeChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for TimeChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        TimeChild::Bold(builder.build())
    }
}
impl From<Button> for TimeChild {
    fn from(child: Button) -> Self {
        TimeChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for TimeChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        TimeChild::Button(builder.build())
    }
}
impl From<Canvas> for TimeChild {
    fn from(child: Canvas) -> Self {
        TimeChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for TimeChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        TimeChild::Canvas(builder.build())
    }
}
impl From<Cite> for TimeChild {
    fn from(child: Cite) -> Self {
        TimeChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for TimeChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        TimeChild::Cite(builder.build())
    }
}
impl From<Code> for TimeChild {
    fn from(child: Code) -> Self {
        TimeChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for TimeChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        TimeChild::Code(builder.build())
    }
}
impl From<Custom> for TimeChild {
    fn from(child: Custom) -> Self {
        TimeChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for TimeChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        TimeChild::Custom(builder.build())
    }
}
impl From<Data> for TimeChild {
    fn from(child: Data) -> Self {
        TimeChild::Data(child)
    }
}
impl From<builders::DataBuilder> for TimeChild {
    fn from(builder: builders::DataBuilder) -> Self {
        TimeChild::Data(builder.build())
    }
}
impl From<DataList> for TimeChild {
    fn from(child: DataList) -> Self {
        TimeChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for TimeChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        TimeChild::DataList(builder.build())
    }
}
impl From<Definition> for TimeChild {
    fn from(child: Definition) -> Self {
        TimeChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for TimeChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        TimeChild::Definition(builder.build())
    }
}
impl From<Deleted> for TimeChild {
    fn from(child: Deleted) -> Self {
        TimeChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for TimeChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        TimeChild::Deleted(builder.build())
    }
}
impl From<Embed> for TimeChild {
    fn from(child: Embed) -> Self {
        TimeChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for TimeChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        TimeChild::Embed(builder.build())
    }
}
impl From<Emphasis> for TimeChild {
    fn from(child: Emphasis) -> Self {
        TimeChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for TimeChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        TimeChild::Emphasis(builder.build())
    }
}
impl From<Image> for TimeChild {
    fn from(child: Image) -> Self {
        TimeChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for TimeChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        TimeChild::Image(builder.build())
    }
}
impl From<InlineFrame> for TimeChild {
    fn from(child: InlineFrame) -> Self {
        TimeChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for TimeChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        TimeChild::InlineFrame(builder.build())
    }
}
impl From<Input> for TimeChild {
    fn from(child: Input) -> Self {
        TimeChild::Input(child)
    }
}
impl From<builders::InputBuilder> for TimeChild {
    fn from(builder: builders::InputBuilder) -> Self {
        TimeChild::Input(builder.build())
    }
}
impl From<Inserted> for TimeChild {
    fn from(child: Inserted) -> Self {
        TimeChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for TimeChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        TimeChild::Inserted(builder.build())
    }
}
impl From<Italic> for TimeChild {
    fn from(child: Italic) -> Self {
        TimeChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for TimeChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        TimeChild::Italic(builder.build())
    }
}
impl From<Keyboard> for TimeChild {
    fn from(child: Keyboard) -> Self {
        TimeChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for TimeChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        TimeChild::Keyboard(builder.build())
    }
}
impl From<Label> for TimeChild {
    fn from(child: Label) -> Self {
        TimeChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for TimeChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        TimeChild::Label(builder.build())
    }
}
impl From<LineBreak> for TimeChild {
    fn from(child: LineBreak) -> Self {
        TimeChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for TimeChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        TimeChild::LineBreak(builder.build())
    }
}
impl From<Link> for TimeChild {
    fn from(child: Link) -> Self {
        TimeChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for TimeChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        TimeChild::Link(builder.build())
    }
}
impl From<Map> for TimeChild {
    fn from(child: Map) -> Self {
        TimeChild::Map(child)
    }
}
impl From<builders::MapBuilder> for TimeChild {
    fn from(builder: builders::MapBuilder) -> Self {
        TimeChild::Map(builder.build())
    }
}
impl From<MapArea> for TimeChild {
    fn from(child: MapArea) -> Self {
        TimeChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for TimeChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        TimeChild::MapArea(builder.build())
    }
}
impl From<Mark> for TimeChild {
    fn from(child: Mark) -> Self {
        TimeChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for TimeChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        TimeChild::Mark(builder.build())
    }
}
impl From<Metadata> for TimeChild {
    fn from(child: Metadata) -> Self {
        TimeChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for TimeChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        TimeChild::Metadata(builder.build())
    }
}
impl From<Meter> for TimeChild {
    fn from(child: Meter) -> Self {
        TimeChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for TimeChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        TimeChild::Meter(builder.build())
    }
}
impl From<NoScript> for TimeChild {
    fn from(child: NoScript) -> Self {
        TimeChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for TimeChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        TimeChild::NoScript(builder.build())
    }
}
impl From<Object> for TimeChild {
    fn from(child: Object) -> Self {
        TimeChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for TimeChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        TimeChild::Object(builder.build())
    }
}
impl From<Output> for TimeChild {
    fn from(child: Output) -> Self {
        TimeChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for TimeChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        TimeChild::Output(builder.build())
    }
}
impl From<Picture> for TimeChild {
    fn from(child: Picture) -> Self {
        TimeChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for TimeChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        TimeChild::Picture(builder.build())
    }
}
impl From<Progress> for TimeChild {
    fn from(child: Progress) -> Self {
        TimeChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for TimeChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        TimeChild::Progress(builder.build())
    }
}
impl From<Quote> for TimeChild {
    fn from(child: Quote) -> Self {
        TimeChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for TimeChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        TimeChild::Quote(builder.build())
    }
}
impl From<Ruby> for TimeChild {
    fn from(child: Ruby) -> Self {
        TimeChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for TimeChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        TimeChild::Ruby(builder.build())
    }
}
impl From<Sample> for TimeChild {
    fn from(child: Sample) -> Self {
        TimeChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for TimeChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        TimeChild::Sample(builder.build())
    }
}
impl From<Script> for TimeChild {
    fn from(child: Script) -> Self {
        TimeChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for TimeChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        TimeChild::Script(builder.build())
    }
}
impl From<Select> for TimeChild {
    fn from(child: Select) -> Self {
        TimeChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for TimeChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        TimeChild::Select(builder.build())
    }
}
impl From<Slot> for TimeChild {
    fn from(child: Slot) -> Self {
        TimeChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for TimeChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        TimeChild::Slot(builder.build())
    }
}
impl From<Small> for TimeChild {
    fn from(child: Small) -> Self {
        TimeChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for TimeChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        TimeChild::Small(builder.build())
    }
}
impl From<Span> for TimeChild {
    fn from(child: Span) -> Self {
        TimeChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for TimeChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        TimeChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for TimeChild {
    fn from(child: StrikeThrough) -> Self {
        TimeChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for TimeChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        TimeChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for TimeChild {
    fn from(child: Strong) -> Self {
        TimeChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for TimeChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        TimeChild::Strong(builder.build())
    }
}
impl From<SubScript> for TimeChild {
    fn from(child: SubScript) -> Self {
        TimeChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for TimeChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        TimeChild::SubScript(builder.build())
    }
}
impl From<SupScript> for TimeChild {
    fn from(child: SupScript) -> Self {
        TimeChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for TimeChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        TimeChild::SupScript(builder.build())
    }
}
impl From<Template> for TimeChild {
    fn from(child: Template) -> Self {
        TimeChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for TimeChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        TimeChild::Template(builder.build())
    }
}
impl From<TextArea> for TimeChild {
    fn from(child: TextArea) -> Self {
        TimeChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for TimeChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        TimeChild::TextArea(builder.build())
    }
}
impl From<Time> for TimeChild {
    fn from(child: Time) -> Self {
        TimeChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for TimeChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        TimeChild::Time(builder.build())
    }
}
impl From<Underline> for TimeChild {
    fn from(child: Underline) -> Self {
        TimeChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for TimeChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        TimeChild::Underline(builder.build())
    }
}
impl From<Variable> for TimeChild {
    fn from(child: Variable) -> Self {
        TimeChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for TimeChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        TimeChild::Variable(builder.build())
    }
}
impl From<Video> for TimeChild {
    fn from(child: Video) -> Self {
        TimeChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for TimeChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        TimeChild::Video(builder.build())
    }
}
impl From<WordBreak> for TimeChild {
    fn from(child: WordBreak) -> Self {
        TimeChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for TimeChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        TimeChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for TimeChild {
    fn from(s: &'static str) -> Self {
        TimeChild::Text(s.into())
    }
}
impl From<String> for TimeChild {
    fn from(s: String) -> Self {
        TimeChild::Text(s.into())
    }
}
impl From<CowStr> for TimeChild {
    fn from(s: CowStr) -> Self {
        TimeChild::Text(s)
    }
}
impl std::fmt::Debug for TimeChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Button(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Code(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Data(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Image(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Input(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Label(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Link(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Map(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Object(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Output(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Script(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Select(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Small(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Span(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Template(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Time(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Video(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            TimeChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for TimeChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Audio(child) => std::fmt::Display::fmt(child, f),
            TimeChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            TimeChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Bold(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Button(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Cite(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Code(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Custom(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Data(child) => std::fmt::Display::fmt(child, f),
            TimeChild::DataList(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Definition(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Embed(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Image(child) => std::fmt::Display::fmt(child, f),
            TimeChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Input(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Italic(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Label(child) => std::fmt::Display::fmt(child, f),
            TimeChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Link(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Map(child) => std::fmt::Display::fmt(child, f),
            TimeChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Mark(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Meter(child) => std::fmt::Display::fmt(child, f),
            TimeChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Object(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Output(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Picture(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Progress(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Quote(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Sample(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Script(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Select(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Slot(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Small(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Span(child) => std::fmt::Display::fmt(child, f),
            TimeChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Strong(child) => std::fmt::Display::fmt(child, f),
            TimeChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            TimeChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Template(child) => std::fmt::Display::fmt(child, f),
            TimeChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Time(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Underline(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Variable(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Video(child) => std::fmt::Display::fmt(child, f),
            TimeChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            TimeChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
