// 🤖 This file is generated!

use crate::*;
/// The `<output>` element's children.
#[derive(Clone)]
pub enum OutputChild {
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
impl From<Abbreviation> for OutputChild {
    fn from(child: Abbreviation) -> Self {
        OutputChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for OutputChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        OutputChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for OutputChild {
    fn from(child: Anchor) -> Self {
        OutputChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for OutputChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        OutputChild::Anchor(builder.build())
    }
}
impl From<Audio> for OutputChild {
    fn from(child: Audio) -> Self {
        OutputChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for OutputChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        OutputChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for OutputChild {
    fn from(child: BidirectionalIsolate) -> Self {
        OutputChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for OutputChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        OutputChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for OutputChild {
    fn from(child: BidirectionalOverride) -> Self {
        OutputChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for OutputChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        OutputChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for OutputChild {
    fn from(child: Bold) -> Self {
        OutputChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for OutputChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        OutputChild::Bold(builder.build())
    }
}
impl From<Button> for OutputChild {
    fn from(child: Button) -> Self {
        OutputChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for OutputChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        OutputChild::Button(builder.build())
    }
}
impl From<Canvas> for OutputChild {
    fn from(child: Canvas) -> Self {
        OutputChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for OutputChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        OutputChild::Canvas(builder.build())
    }
}
impl From<Cite> for OutputChild {
    fn from(child: Cite) -> Self {
        OutputChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for OutputChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        OutputChild::Cite(builder.build())
    }
}
impl From<Code> for OutputChild {
    fn from(child: Code) -> Self {
        OutputChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for OutputChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        OutputChild::Code(builder.build())
    }
}
impl From<Custom> for OutputChild {
    fn from(child: Custom) -> Self {
        OutputChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for OutputChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        OutputChild::Custom(builder.build())
    }
}
impl From<Data> for OutputChild {
    fn from(child: Data) -> Self {
        OutputChild::Data(child)
    }
}
impl From<builders::DataBuilder> for OutputChild {
    fn from(builder: builders::DataBuilder) -> Self {
        OutputChild::Data(builder.build())
    }
}
impl From<DataList> for OutputChild {
    fn from(child: DataList) -> Self {
        OutputChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for OutputChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        OutputChild::DataList(builder.build())
    }
}
impl From<Definition> for OutputChild {
    fn from(child: Definition) -> Self {
        OutputChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for OutputChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        OutputChild::Definition(builder.build())
    }
}
impl From<Deleted> for OutputChild {
    fn from(child: Deleted) -> Self {
        OutputChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for OutputChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        OutputChild::Deleted(builder.build())
    }
}
impl From<Embed> for OutputChild {
    fn from(child: Embed) -> Self {
        OutputChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for OutputChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        OutputChild::Embed(builder.build())
    }
}
impl From<Emphasis> for OutputChild {
    fn from(child: Emphasis) -> Self {
        OutputChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for OutputChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        OutputChild::Emphasis(builder.build())
    }
}
impl From<Image> for OutputChild {
    fn from(child: Image) -> Self {
        OutputChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for OutputChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        OutputChild::Image(builder.build())
    }
}
impl From<InlineFrame> for OutputChild {
    fn from(child: InlineFrame) -> Self {
        OutputChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for OutputChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        OutputChild::InlineFrame(builder.build())
    }
}
impl From<Input> for OutputChild {
    fn from(child: Input) -> Self {
        OutputChild::Input(child)
    }
}
impl From<builders::InputBuilder> for OutputChild {
    fn from(builder: builders::InputBuilder) -> Self {
        OutputChild::Input(builder.build())
    }
}
impl From<Inserted> for OutputChild {
    fn from(child: Inserted) -> Self {
        OutputChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for OutputChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        OutputChild::Inserted(builder.build())
    }
}
impl From<Italic> for OutputChild {
    fn from(child: Italic) -> Self {
        OutputChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for OutputChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        OutputChild::Italic(builder.build())
    }
}
impl From<Keyboard> for OutputChild {
    fn from(child: Keyboard) -> Self {
        OutputChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for OutputChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        OutputChild::Keyboard(builder.build())
    }
}
impl From<Label> for OutputChild {
    fn from(child: Label) -> Self {
        OutputChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for OutputChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        OutputChild::Label(builder.build())
    }
}
impl From<LineBreak> for OutputChild {
    fn from(child: LineBreak) -> Self {
        OutputChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for OutputChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        OutputChild::LineBreak(builder.build())
    }
}
impl From<Link> for OutputChild {
    fn from(child: Link) -> Self {
        OutputChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for OutputChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        OutputChild::Link(builder.build())
    }
}
impl From<Map> for OutputChild {
    fn from(child: Map) -> Self {
        OutputChild::Map(child)
    }
}
impl From<builders::MapBuilder> for OutputChild {
    fn from(builder: builders::MapBuilder) -> Self {
        OutputChild::Map(builder.build())
    }
}
impl From<MapArea> for OutputChild {
    fn from(child: MapArea) -> Self {
        OutputChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for OutputChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        OutputChild::MapArea(builder.build())
    }
}
impl From<Mark> for OutputChild {
    fn from(child: Mark) -> Self {
        OutputChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for OutputChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        OutputChild::Mark(builder.build())
    }
}
impl From<Metadata> for OutputChild {
    fn from(child: Metadata) -> Self {
        OutputChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for OutputChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        OutputChild::Metadata(builder.build())
    }
}
impl From<Meter> for OutputChild {
    fn from(child: Meter) -> Self {
        OutputChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for OutputChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        OutputChild::Meter(builder.build())
    }
}
impl From<NoScript> for OutputChild {
    fn from(child: NoScript) -> Self {
        OutputChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for OutputChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        OutputChild::NoScript(builder.build())
    }
}
impl From<Object> for OutputChild {
    fn from(child: Object) -> Self {
        OutputChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for OutputChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        OutputChild::Object(builder.build())
    }
}
impl From<Output> for OutputChild {
    fn from(child: Output) -> Self {
        OutputChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for OutputChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        OutputChild::Output(builder.build())
    }
}
impl From<Picture> for OutputChild {
    fn from(child: Picture) -> Self {
        OutputChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for OutputChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        OutputChild::Picture(builder.build())
    }
}
impl From<Progress> for OutputChild {
    fn from(child: Progress) -> Self {
        OutputChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for OutputChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        OutputChild::Progress(builder.build())
    }
}
impl From<Quote> for OutputChild {
    fn from(child: Quote) -> Self {
        OutputChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for OutputChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        OutputChild::Quote(builder.build())
    }
}
impl From<Ruby> for OutputChild {
    fn from(child: Ruby) -> Self {
        OutputChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for OutputChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        OutputChild::Ruby(builder.build())
    }
}
impl From<Sample> for OutputChild {
    fn from(child: Sample) -> Self {
        OutputChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for OutputChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        OutputChild::Sample(builder.build())
    }
}
impl From<Script> for OutputChild {
    fn from(child: Script) -> Self {
        OutputChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for OutputChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        OutputChild::Script(builder.build())
    }
}
impl From<Select> for OutputChild {
    fn from(child: Select) -> Self {
        OutputChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for OutputChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        OutputChild::Select(builder.build())
    }
}
impl From<Slot> for OutputChild {
    fn from(child: Slot) -> Self {
        OutputChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for OutputChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        OutputChild::Slot(builder.build())
    }
}
impl From<Small> for OutputChild {
    fn from(child: Small) -> Self {
        OutputChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for OutputChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        OutputChild::Small(builder.build())
    }
}
impl From<Span> for OutputChild {
    fn from(child: Span) -> Self {
        OutputChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for OutputChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        OutputChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for OutputChild {
    fn from(child: StrikeThrough) -> Self {
        OutputChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for OutputChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        OutputChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for OutputChild {
    fn from(child: Strong) -> Self {
        OutputChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for OutputChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        OutputChild::Strong(builder.build())
    }
}
impl From<SubScript> for OutputChild {
    fn from(child: SubScript) -> Self {
        OutputChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for OutputChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        OutputChild::SubScript(builder.build())
    }
}
impl From<SupScript> for OutputChild {
    fn from(child: SupScript) -> Self {
        OutputChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for OutputChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        OutputChild::SupScript(builder.build())
    }
}
impl From<Template> for OutputChild {
    fn from(child: Template) -> Self {
        OutputChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for OutputChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        OutputChild::Template(builder.build())
    }
}
impl From<TextArea> for OutputChild {
    fn from(child: TextArea) -> Self {
        OutputChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for OutputChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        OutputChild::TextArea(builder.build())
    }
}
impl From<Time> for OutputChild {
    fn from(child: Time) -> Self {
        OutputChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for OutputChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        OutputChild::Time(builder.build())
    }
}
impl From<Underline> for OutputChild {
    fn from(child: Underline) -> Self {
        OutputChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for OutputChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        OutputChild::Underline(builder.build())
    }
}
impl From<Variable> for OutputChild {
    fn from(child: Variable) -> Self {
        OutputChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for OutputChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        OutputChild::Variable(builder.build())
    }
}
impl From<Video> for OutputChild {
    fn from(child: Video) -> Self {
        OutputChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for OutputChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        OutputChild::Video(builder.build())
    }
}
impl From<WordBreak> for OutputChild {
    fn from(child: WordBreak) -> Self {
        OutputChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for OutputChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        OutputChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for OutputChild {
    fn from(s: &'static str) -> Self {
        OutputChild::Text(s.into())
    }
}
impl From<String> for OutputChild {
    fn from(s: String) -> Self {
        OutputChild::Text(s.into())
    }
}
impl From<CowStr> for OutputChild {
    fn from(s: CowStr) -> Self {
        OutputChild::Text(s)
    }
}
impl std::fmt::Debug for OutputChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Button(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Code(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Data(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Image(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Input(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Label(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Link(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Map(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Object(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Output(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Script(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Select(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Small(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Span(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Template(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Time(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Video(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            OutputChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for OutputChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Audio(child) => std::fmt::Display::fmt(child, f),
            OutputChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            OutputChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Bold(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Button(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Cite(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Code(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Custom(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Data(child) => std::fmt::Display::fmt(child, f),
            OutputChild::DataList(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Definition(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Embed(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Image(child) => std::fmt::Display::fmt(child, f),
            OutputChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Input(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Italic(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Label(child) => std::fmt::Display::fmt(child, f),
            OutputChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Link(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Map(child) => std::fmt::Display::fmt(child, f),
            OutputChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Mark(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Meter(child) => std::fmt::Display::fmt(child, f),
            OutputChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Object(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Output(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Picture(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Progress(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Quote(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Sample(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Script(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Select(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Slot(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Small(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Span(child) => std::fmt::Display::fmt(child, f),
            OutputChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Strong(child) => std::fmt::Display::fmt(child, f),
            OutputChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            OutputChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Template(child) => std::fmt::Display::fmt(child, f),
            OutputChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Time(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Underline(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Variable(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Video(child) => std::fmt::Display::fmt(child, f),
            OutputChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            OutputChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
