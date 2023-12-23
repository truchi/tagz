// 🤖 This file is generated!

use crate::*;
/// The `<small>` element's children.
#[derive(Clone)]
pub enum SmallChild {
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
impl From<Abbreviation> for SmallChild {
    fn from(child: Abbreviation) -> Self {
        SmallChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for SmallChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        SmallChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for SmallChild {
    fn from(child: Anchor) -> Self {
        SmallChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for SmallChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        SmallChild::Anchor(builder.build())
    }
}
impl From<Audio> for SmallChild {
    fn from(child: Audio) -> Self {
        SmallChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for SmallChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        SmallChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for SmallChild {
    fn from(child: BidirectionalIsolate) -> Self {
        SmallChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for SmallChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        SmallChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for SmallChild {
    fn from(child: BidirectionalOverride) -> Self {
        SmallChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for SmallChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        SmallChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for SmallChild {
    fn from(child: Bold) -> Self {
        SmallChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for SmallChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        SmallChild::Bold(builder.build())
    }
}
impl From<Button> for SmallChild {
    fn from(child: Button) -> Self {
        SmallChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for SmallChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        SmallChild::Button(builder.build())
    }
}
impl From<Canvas> for SmallChild {
    fn from(child: Canvas) -> Self {
        SmallChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for SmallChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        SmallChild::Canvas(builder.build())
    }
}
impl From<Cite> for SmallChild {
    fn from(child: Cite) -> Self {
        SmallChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for SmallChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        SmallChild::Cite(builder.build())
    }
}
impl From<Code> for SmallChild {
    fn from(child: Code) -> Self {
        SmallChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for SmallChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        SmallChild::Code(builder.build())
    }
}
impl From<Custom> for SmallChild {
    fn from(child: Custom) -> Self {
        SmallChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for SmallChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        SmallChild::Custom(builder.build())
    }
}
impl From<Data> for SmallChild {
    fn from(child: Data) -> Self {
        SmallChild::Data(child)
    }
}
impl From<builders::DataBuilder> for SmallChild {
    fn from(builder: builders::DataBuilder) -> Self {
        SmallChild::Data(builder.build())
    }
}
impl From<DataList> for SmallChild {
    fn from(child: DataList) -> Self {
        SmallChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for SmallChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        SmallChild::DataList(builder.build())
    }
}
impl From<Definition> for SmallChild {
    fn from(child: Definition) -> Self {
        SmallChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for SmallChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        SmallChild::Definition(builder.build())
    }
}
impl From<Deleted> for SmallChild {
    fn from(child: Deleted) -> Self {
        SmallChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for SmallChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        SmallChild::Deleted(builder.build())
    }
}
impl From<Embed> for SmallChild {
    fn from(child: Embed) -> Self {
        SmallChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for SmallChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        SmallChild::Embed(builder.build())
    }
}
impl From<Emphasis> for SmallChild {
    fn from(child: Emphasis) -> Self {
        SmallChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for SmallChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        SmallChild::Emphasis(builder.build())
    }
}
impl From<Image> for SmallChild {
    fn from(child: Image) -> Self {
        SmallChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for SmallChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        SmallChild::Image(builder.build())
    }
}
impl From<InlineFrame> for SmallChild {
    fn from(child: InlineFrame) -> Self {
        SmallChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for SmallChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        SmallChild::InlineFrame(builder.build())
    }
}
impl From<Input> for SmallChild {
    fn from(child: Input) -> Self {
        SmallChild::Input(child)
    }
}
impl From<builders::InputBuilder> for SmallChild {
    fn from(builder: builders::InputBuilder) -> Self {
        SmallChild::Input(builder.build())
    }
}
impl From<Inserted> for SmallChild {
    fn from(child: Inserted) -> Self {
        SmallChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for SmallChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        SmallChild::Inserted(builder.build())
    }
}
impl From<Italic> for SmallChild {
    fn from(child: Italic) -> Self {
        SmallChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for SmallChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        SmallChild::Italic(builder.build())
    }
}
impl From<Keyboard> for SmallChild {
    fn from(child: Keyboard) -> Self {
        SmallChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for SmallChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        SmallChild::Keyboard(builder.build())
    }
}
impl From<Label> for SmallChild {
    fn from(child: Label) -> Self {
        SmallChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for SmallChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        SmallChild::Label(builder.build())
    }
}
impl From<LineBreak> for SmallChild {
    fn from(child: LineBreak) -> Self {
        SmallChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for SmallChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        SmallChild::LineBreak(builder.build())
    }
}
impl From<Link> for SmallChild {
    fn from(child: Link) -> Self {
        SmallChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for SmallChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        SmallChild::Link(builder.build())
    }
}
impl From<Map> for SmallChild {
    fn from(child: Map) -> Self {
        SmallChild::Map(child)
    }
}
impl From<builders::MapBuilder> for SmallChild {
    fn from(builder: builders::MapBuilder) -> Self {
        SmallChild::Map(builder.build())
    }
}
impl From<MapArea> for SmallChild {
    fn from(child: MapArea) -> Self {
        SmallChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for SmallChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        SmallChild::MapArea(builder.build())
    }
}
impl From<Mark> for SmallChild {
    fn from(child: Mark) -> Self {
        SmallChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for SmallChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        SmallChild::Mark(builder.build())
    }
}
impl From<Metadata> for SmallChild {
    fn from(child: Metadata) -> Self {
        SmallChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for SmallChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        SmallChild::Metadata(builder.build())
    }
}
impl From<Meter> for SmallChild {
    fn from(child: Meter) -> Self {
        SmallChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for SmallChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        SmallChild::Meter(builder.build())
    }
}
impl From<NoScript> for SmallChild {
    fn from(child: NoScript) -> Self {
        SmallChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for SmallChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        SmallChild::NoScript(builder.build())
    }
}
impl From<Object> for SmallChild {
    fn from(child: Object) -> Self {
        SmallChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for SmallChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        SmallChild::Object(builder.build())
    }
}
impl From<Output> for SmallChild {
    fn from(child: Output) -> Self {
        SmallChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for SmallChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        SmallChild::Output(builder.build())
    }
}
impl From<Picture> for SmallChild {
    fn from(child: Picture) -> Self {
        SmallChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for SmallChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        SmallChild::Picture(builder.build())
    }
}
impl From<Progress> for SmallChild {
    fn from(child: Progress) -> Self {
        SmallChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for SmallChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        SmallChild::Progress(builder.build())
    }
}
impl From<Quote> for SmallChild {
    fn from(child: Quote) -> Self {
        SmallChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for SmallChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        SmallChild::Quote(builder.build())
    }
}
impl From<Ruby> for SmallChild {
    fn from(child: Ruby) -> Self {
        SmallChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for SmallChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        SmallChild::Ruby(builder.build())
    }
}
impl From<Sample> for SmallChild {
    fn from(child: Sample) -> Self {
        SmallChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for SmallChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        SmallChild::Sample(builder.build())
    }
}
impl From<Script> for SmallChild {
    fn from(child: Script) -> Self {
        SmallChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for SmallChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        SmallChild::Script(builder.build())
    }
}
impl From<Select> for SmallChild {
    fn from(child: Select) -> Self {
        SmallChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for SmallChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        SmallChild::Select(builder.build())
    }
}
impl From<Slot> for SmallChild {
    fn from(child: Slot) -> Self {
        SmallChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for SmallChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        SmallChild::Slot(builder.build())
    }
}
impl From<Small> for SmallChild {
    fn from(child: Small) -> Self {
        SmallChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for SmallChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        SmallChild::Small(builder.build())
    }
}
impl From<Span> for SmallChild {
    fn from(child: Span) -> Self {
        SmallChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for SmallChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        SmallChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for SmallChild {
    fn from(child: StrikeThrough) -> Self {
        SmallChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for SmallChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        SmallChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for SmallChild {
    fn from(child: Strong) -> Self {
        SmallChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for SmallChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        SmallChild::Strong(builder.build())
    }
}
impl From<SubScript> for SmallChild {
    fn from(child: SubScript) -> Self {
        SmallChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for SmallChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        SmallChild::SubScript(builder.build())
    }
}
impl From<SupScript> for SmallChild {
    fn from(child: SupScript) -> Self {
        SmallChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for SmallChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        SmallChild::SupScript(builder.build())
    }
}
impl From<Template> for SmallChild {
    fn from(child: Template) -> Self {
        SmallChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for SmallChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        SmallChild::Template(builder.build())
    }
}
impl From<TextArea> for SmallChild {
    fn from(child: TextArea) -> Self {
        SmallChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for SmallChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        SmallChild::TextArea(builder.build())
    }
}
impl From<Time> for SmallChild {
    fn from(child: Time) -> Self {
        SmallChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for SmallChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        SmallChild::Time(builder.build())
    }
}
impl From<Underline> for SmallChild {
    fn from(child: Underline) -> Self {
        SmallChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for SmallChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        SmallChild::Underline(builder.build())
    }
}
impl From<Variable> for SmallChild {
    fn from(child: Variable) -> Self {
        SmallChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for SmallChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        SmallChild::Variable(builder.build())
    }
}
impl From<Video> for SmallChild {
    fn from(child: Video) -> Self {
        SmallChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for SmallChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        SmallChild::Video(builder.build())
    }
}
impl From<WordBreak> for SmallChild {
    fn from(child: WordBreak) -> Self {
        SmallChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for SmallChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        SmallChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for SmallChild {
    fn from(s: &'static str) -> Self {
        SmallChild::Text(s.into())
    }
}
impl From<String> for SmallChild {
    fn from(s: String) -> Self {
        SmallChild::Text(s.into())
    }
}
impl From<CowStr> for SmallChild {
    fn from(s: CowStr) -> Self {
        SmallChild::Text(s)
    }
}
impl std::fmt::Debug for SmallChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmallChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Button(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Code(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Data(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Image(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Input(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Label(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Link(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Map(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Object(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Output(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Script(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Select(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Small(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Span(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Template(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Time(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Video(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            SmallChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for SmallChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmallChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Audio(child) => std::fmt::Display::fmt(child, f),
            SmallChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            SmallChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Bold(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Button(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Cite(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Code(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Custom(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Data(child) => std::fmt::Display::fmt(child, f),
            SmallChild::DataList(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Definition(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Embed(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Image(child) => std::fmt::Display::fmt(child, f),
            SmallChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Input(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Italic(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Label(child) => std::fmt::Display::fmt(child, f),
            SmallChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Link(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Map(child) => std::fmt::Display::fmt(child, f),
            SmallChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Mark(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Meter(child) => std::fmt::Display::fmt(child, f),
            SmallChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Object(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Output(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Picture(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Progress(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Quote(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Sample(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Script(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Select(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Slot(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Small(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Span(child) => std::fmt::Display::fmt(child, f),
            SmallChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Strong(child) => std::fmt::Display::fmt(child, f),
            SmallChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            SmallChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Template(child) => std::fmt::Display::fmt(child, f),
            SmallChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Time(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Underline(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Variable(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Video(child) => std::fmt::Display::fmt(child, f),
            SmallChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            SmallChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
