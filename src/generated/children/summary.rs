// 🤖 This file is generated!

use crate::*;
/// The `<summary>` element's children.
#[derive(Clone)]
pub enum SummaryChild {
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
impl From<Abbreviation> for SummaryChild {
    fn from(child: Abbreviation) -> Self {
        SummaryChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for SummaryChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        SummaryChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for SummaryChild {
    fn from(child: Anchor) -> Self {
        SummaryChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for SummaryChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        SummaryChild::Anchor(builder.build())
    }
}
impl From<Audio> for SummaryChild {
    fn from(child: Audio) -> Self {
        SummaryChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for SummaryChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        SummaryChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for SummaryChild {
    fn from(child: BidirectionalIsolate) -> Self {
        SummaryChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for SummaryChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        SummaryChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for SummaryChild {
    fn from(child: BidirectionalOverride) -> Self {
        SummaryChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for SummaryChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        SummaryChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for SummaryChild {
    fn from(child: Bold) -> Self {
        SummaryChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for SummaryChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        SummaryChild::Bold(builder.build())
    }
}
impl From<Button> for SummaryChild {
    fn from(child: Button) -> Self {
        SummaryChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for SummaryChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        SummaryChild::Button(builder.build())
    }
}
impl From<Canvas> for SummaryChild {
    fn from(child: Canvas) -> Self {
        SummaryChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for SummaryChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        SummaryChild::Canvas(builder.build())
    }
}
impl From<Cite> for SummaryChild {
    fn from(child: Cite) -> Self {
        SummaryChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for SummaryChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        SummaryChild::Cite(builder.build())
    }
}
impl From<Code> for SummaryChild {
    fn from(child: Code) -> Self {
        SummaryChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for SummaryChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        SummaryChild::Code(builder.build())
    }
}
impl From<Custom> for SummaryChild {
    fn from(child: Custom) -> Self {
        SummaryChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for SummaryChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        SummaryChild::Custom(builder.build())
    }
}
impl From<Data> for SummaryChild {
    fn from(child: Data) -> Self {
        SummaryChild::Data(child)
    }
}
impl From<builders::DataBuilder> for SummaryChild {
    fn from(builder: builders::DataBuilder) -> Self {
        SummaryChild::Data(builder.build())
    }
}
impl From<DataList> for SummaryChild {
    fn from(child: DataList) -> Self {
        SummaryChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for SummaryChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        SummaryChild::DataList(builder.build())
    }
}
impl From<Definition> for SummaryChild {
    fn from(child: Definition) -> Self {
        SummaryChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for SummaryChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        SummaryChild::Definition(builder.build())
    }
}
impl From<Deleted> for SummaryChild {
    fn from(child: Deleted) -> Self {
        SummaryChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for SummaryChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        SummaryChild::Deleted(builder.build())
    }
}
impl From<Embed> for SummaryChild {
    fn from(child: Embed) -> Self {
        SummaryChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for SummaryChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        SummaryChild::Embed(builder.build())
    }
}
impl From<Emphasis> for SummaryChild {
    fn from(child: Emphasis) -> Self {
        SummaryChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for SummaryChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        SummaryChild::Emphasis(builder.build())
    }
}
impl From<Image> for SummaryChild {
    fn from(child: Image) -> Self {
        SummaryChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for SummaryChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        SummaryChild::Image(builder.build())
    }
}
impl From<InlineFrame> for SummaryChild {
    fn from(child: InlineFrame) -> Self {
        SummaryChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for SummaryChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        SummaryChild::InlineFrame(builder.build())
    }
}
impl From<Input> for SummaryChild {
    fn from(child: Input) -> Self {
        SummaryChild::Input(child)
    }
}
impl From<builders::InputBuilder> for SummaryChild {
    fn from(builder: builders::InputBuilder) -> Self {
        SummaryChild::Input(builder.build())
    }
}
impl From<Inserted> for SummaryChild {
    fn from(child: Inserted) -> Self {
        SummaryChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for SummaryChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        SummaryChild::Inserted(builder.build())
    }
}
impl From<Italic> for SummaryChild {
    fn from(child: Italic) -> Self {
        SummaryChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for SummaryChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        SummaryChild::Italic(builder.build())
    }
}
impl From<Keyboard> for SummaryChild {
    fn from(child: Keyboard) -> Self {
        SummaryChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for SummaryChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        SummaryChild::Keyboard(builder.build())
    }
}
impl From<Label> for SummaryChild {
    fn from(child: Label) -> Self {
        SummaryChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for SummaryChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        SummaryChild::Label(builder.build())
    }
}
impl From<LineBreak> for SummaryChild {
    fn from(child: LineBreak) -> Self {
        SummaryChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for SummaryChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        SummaryChild::LineBreak(builder.build())
    }
}
impl From<Link> for SummaryChild {
    fn from(child: Link) -> Self {
        SummaryChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for SummaryChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        SummaryChild::Link(builder.build())
    }
}
impl From<Map> for SummaryChild {
    fn from(child: Map) -> Self {
        SummaryChild::Map(child)
    }
}
impl From<builders::MapBuilder> for SummaryChild {
    fn from(builder: builders::MapBuilder) -> Self {
        SummaryChild::Map(builder.build())
    }
}
impl From<MapArea> for SummaryChild {
    fn from(child: MapArea) -> Self {
        SummaryChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for SummaryChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        SummaryChild::MapArea(builder.build())
    }
}
impl From<Mark> for SummaryChild {
    fn from(child: Mark) -> Self {
        SummaryChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for SummaryChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        SummaryChild::Mark(builder.build())
    }
}
impl From<Metadata> for SummaryChild {
    fn from(child: Metadata) -> Self {
        SummaryChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for SummaryChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        SummaryChild::Metadata(builder.build())
    }
}
impl From<Meter> for SummaryChild {
    fn from(child: Meter) -> Self {
        SummaryChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for SummaryChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        SummaryChild::Meter(builder.build())
    }
}
impl From<NoScript> for SummaryChild {
    fn from(child: NoScript) -> Self {
        SummaryChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for SummaryChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        SummaryChild::NoScript(builder.build())
    }
}
impl From<Object> for SummaryChild {
    fn from(child: Object) -> Self {
        SummaryChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for SummaryChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        SummaryChild::Object(builder.build())
    }
}
impl From<Output> for SummaryChild {
    fn from(child: Output) -> Self {
        SummaryChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for SummaryChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        SummaryChild::Output(builder.build())
    }
}
impl From<Picture> for SummaryChild {
    fn from(child: Picture) -> Self {
        SummaryChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for SummaryChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        SummaryChild::Picture(builder.build())
    }
}
impl From<Progress> for SummaryChild {
    fn from(child: Progress) -> Self {
        SummaryChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for SummaryChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        SummaryChild::Progress(builder.build())
    }
}
impl From<Quote> for SummaryChild {
    fn from(child: Quote) -> Self {
        SummaryChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for SummaryChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        SummaryChild::Quote(builder.build())
    }
}
impl From<Ruby> for SummaryChild {
    fn from(child: Ruby) -> Self {
        SummaryChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for SummaryChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        SummaryChild::Ruby(builder.build())
    }
}
impl From<Sample> for SummaryChild {
    fn from(child: Sample) -> Self {
        SummaryChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for SummaryChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        SummaryChild::Sample(builder.build())
    }
}
impl From<Script> for SummaryChild {
    fn from(child: Script) -> Self {
        SummaryChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for SummaryChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        SummaryChild::Script(builder.build())
    }
}
impl From<Select> for SummaryChild {
    fn from(child: Select) -> Self {
        SummaryChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for SummaryChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        SummaryChild::Select(builder.build())
    }
}
impl From<Slot> for SummaryChild {
    fn from(child: Slot) -> Self {
        SummaryChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for SummaryChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        SummaryChild::Slot(builder.build())
    }
}
impl From<Small> for SummaryChild {
    fn from(child: Small) -> Self {
        SummaryChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for SummaryChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        SummaryChild::Small(builder.build())
    }
}
impl From<Span> for SummaryChild {
    fn from(child: Span) -> Self {
        SummaryChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for SummaryChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        SummaryChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for SummaryChild {
    fn from(child: StrikeThrough) -> Self {
        SummaryChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for SummaryChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        SummaryChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for SummaryChild {
    fn from(child: Strong) -> Self {
        SummaryChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for SummaryChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        SummaryChild::Strong(builder.build())
    }
}
impl From<SubScript> for SummaryChild {
    fn from(child: SubScript) -> Self {
        SummaryChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for SummaryChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        SummaryChild::SubScript(builder.build())
    }
}
impl From<SupScript> for SummaryChild {
    fn from(child: SupScript) -> Self {
        SummaryChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for SummaryChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        SummaryChild::SupScript(builder.build())
    }
}
impl From<Template> for SummaryChild {
    fn from(child: Template) -> Self {
        SummaryChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for SummaryChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        SummaryChild::Template(builder.build())
    }
}
impl From<TextArea> for SummaryChild {
    fn from(child: TextArea) -> Self {
        SummaryChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for SummaryChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        SummaryChild::TextArea(builder.build())
    }
}
impl From<Time> for SummaryChild {
    fn from(child: Time) -> Self {
        SummaryChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for SummaryChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        SummaryChild::Time(builder.build())
    }
}
impl From<Underline> for SummaryChild {
    fn from(child: Underline) -> Self {
        SummaryChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for SummaryChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        SummaryChild::Underline(builder.build())
    }
}
impl From<Variable> for SummaryChild {
    fn from(child: Variable) -> Self {
        SummaryChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for SummaryChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        SummaryChild::Variable(builder.build())
    }
}
impl From<Video> for SummaryChild {
    fn from(child: Video) -> Self {
        SummaryChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for SummaryChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        SummaryChild::Video(builder.build())
    }
}
impl From<WordBreak> for SummaryChild {
    fn from(child: WordBreak) -> Self {
        SummaryChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for SummaryChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        SummaryChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for SummaryChild {
    fn from(s: &'static str) -> Self {
        SummaryChild::Text(s.into())
    }
}
impl From<String> for SummaryChild {
    fn from(s: String) -> Self {
        SummaryChild::Text(s.into())
    }
}
impl From<CowStr> for SummaryChild {
    fn from(s: CowStr) -> Self {
        SummaryChild::Text(s)
    }
}
impl std::fmt::Debug for SummaryChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SummaryChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Button(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Code(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Data(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Image(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Input(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Label(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Link(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Map(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Object(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Output(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Script(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Select(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Small(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Span(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Template(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Time(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Video(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            SummaryChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for SummaryChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SummaryChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Audio(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            SummaryChild::Bold(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Button(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Cite(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Code(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Custom(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Data(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::DataList(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Definition(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Embed(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Image(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Input(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Italic(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Label(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Link(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Map(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Mark(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Meter(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Object(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Output(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Picture(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Progress(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Quote(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Sample(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Script(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Select(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Slot(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Small(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Span(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Strong(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Template(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Time(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Underline(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Variable(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Video(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            SummaryChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
