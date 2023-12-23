// 🤖 This file is generated!

use crate::*;
/// The `<meter>` element's children.
#[derive(Clone)]
pub enum MeterChild {
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
impl From<Abbreviation> for MeterChild {
    fn from(child: Abbreviation) -> Self {
        MeterChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for MeterChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        MeterChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for MeterChild {
    fn from(child: Anchor) -> Self {
        MeterChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for MeterChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        MeterChild::Anchor(builder.build())
    }
}
impl From<Audio> for MeterChild {
    fn from(child: Audio) -> Self {
        MeterChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for MeterChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        MeterChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for MeterChild {
    fn from(child: BidirectionalIsolate) -> Self {
        MeterChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for MeterChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        MeterChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for MeterChild {
    fn from(child: BidirectionalOverride) -> Self {
        MeterChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for MeterChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        MeterChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for MeterChild {
    fn from(child: Bold) -> Self {
        MeterChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for MeterChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        MeterChild::Bold(builder.build())
    }
}
impl From<Button> for MeterChild {
    fn from(child: Button) -> Self {
        MeterChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for MeterChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        MeterChild::Button(builder.build())
    }
}
impl From<Canvas> for MeterChild {
    fn from(child: Canvas) -> Self {
        MeterChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for MeterChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        MeterChild::Canvas(builder.build())
    }
}
impl From<Cite> for MeterChild {
    fn from(child: Cite) -> Self {
        MeterChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for MeterChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        MeterChild::Cite(builder.build())
    }
}
impl From<Code> for MeterChild {
    fn from(child: Code) -> Self {
        MeterChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for MeterChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        MeterChild::Code(builder.build())
    }
}
impl From<Custom> for MeterChild {
    fn from(child: Custom) -> Self {
        MeterChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for MeterChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        MeterChild::Custom(builder.build())
    }
}
impl From<Data> for MeterChild {
    fn from(child: Data) -> Self {
        MeterChild::Data(child)
    }
}
impl From<builders::DataBuilder> for MeterChild {
    fn from(builder: builders::DataBuilder) -> Self {
        MeterChild::Data(builder.build())
    }
}
impl From<DataList> for MeterChild {
    fn from(child: DataList) -> Self {
        MeterChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for MeterChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        MeterChild::DataList(builder.build())
    }
}
impl From<Definition> for MeterChild {
    fn from(child: Definition) -> Self {
        MeterChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for MeterChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        MeterChild::Definition(builder.build())
    }
}
impl From<Deleted> for MeterChild {
    fn from(child: Deleted) -> Self {
        MeterChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for MeterChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        MeterChild::Deleted(builder.build())
    }
}
impl From<Embed> for MeterChild {
    fn from(child: Embed) -> Self {
        MeterChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for MeterChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        MeterChild::Embed(builder.build())
    }
}
impl From<Emphasis> for MeterChild {
    fn from(child: Emphasis) -> Self {
        MeterChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for MeterChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        MeterChild::Emphasis(builder.build())
    }
}
impl From<Image> for MeterChild {
    fn from(child: Image) -> Self {
        MeterChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for MeterChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        MeterChild::Image(builder.build())
    }
}
impl From<InlineFrame> for MeterChild {
    fn from(child: InlineFrame) -> Self {
        MeterChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for MeterChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        MeterChild::InlineFrame(builder.build())
    }
}
impl From<Input> for MeterChild {
    fn from(child: Input) -> Self {
        MeterChild::Input(child)
    }
}
impl From<builders::InputBuilder> for MeterChild {
    fn from(builder: builders::InputBuilder) -> Self {
        MeterChild::Input(builder.build())
    }
}
impl From<Inserted> for MeterChild {
    fn from(child: Inserted) -> Self {
        MeterChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for MeterChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        MeterChild::Inserted(builder.build())
    }
}
impl From<Italic> for MeterChild {
    fn from(child: Italic) -> Self {
        MeterChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for MeterChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        MeterChild::Italic(builder.build())
    }
}
impl From<Keyboard> for MeterChild {
    fn from(child: Keyboard) -> Self {
        MeterChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for MeterChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        MeterChild::Keyboard(builder.build())
    }
}
impl From<Label> for MeterChild {
    fn from(child: Label) -> Self {
        MeterChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for MeterChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        MeterChild::Label(builder.build())
    }
}
impl From<LineBreak> for MeterChild {
    fn from(child: LineBreak) -> Self {
        MeterChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for MeterChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        MeterChild::LineBreak(builder.build())
    }
}
impl From<Link> for MeterChild {
    fn from(child: Link) -> Self {
        MeterChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for MeterChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        MeterChild::Link(builder.build())
    }
}
impl From<Map> for MeterChild {
    fn from(child: Map) -> Self {
        MeterChild::Map(child)
    }
}
impl From<builders::MapBuilder> for MeterChild {
    fn from(builder: builders::MapBuilder) -> Self {
        MeterChild::Map(builder.build())
    }
}
impl From<MapArea> for MeterChild {
    fn from(child: MapArea) -> Self {
        MeterChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for MeterChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        MeterChild::MapArea(builder.build())
    }
}
impl From<Mark> for MeterChild {
    fn from(child: Mark) -> Self {
        MeterChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for MeterChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        MeterChild::Mark(builder.build())
    }
}
impl From<Metadata> for MeterChild {
    fn from(child: Metadata) -> Self {
        MeterChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for MeterChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        MeterChild::Metadata(builder.build())
    }
}
impl From<Meter> for MeterChild {
    fn from(child: Meter) -> Self {
        MeterChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for MeterChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        MeterChild::Meter(builder.build())
    }
}
impl From<NoScript> for MeterChild {
    fn from(child: NoScript) -> Self {
        MeterChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for MeterChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        MeterChild::NoScript(builder.build())
    }
}
impl From<Object> for MeterChild {
    fn from(child: Object) -> Self {
        MeterChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for MeterChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        MeterChild::Object(builder.build())
    }
}
impl From<Output> for MeterChild {
    fn from(child: Output) -> Self {
        MeterChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for MeterChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        MeterChild::Output(builder.build())
    }
}
impl From<Picture> for MeterChild {
    fn from(child: Picture) -> Self {
        MeterChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for MeterChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        MeterChild::Picture(builder.build())
    }
}
impl From<Progress> for MeterChild {
    fn from(child: Progress) -> Self {
        MeterChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for MeterChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        MeterChild::Progress(builder.build())
    }
}
impl From<Quote> for MeterChild {
    fn from(child: Quote) -> Self {
        MeterChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for MeterChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        MeterChild::Quote(builder.build())
    }
}
impl From<Ruby> for MeterChild {
    fn from(child: Ruby) -> Self {
        MeterChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for MeterChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        MeterChild::Ruby(builder.build())
    }
}
impl From<Sample> for MeterChild {
    fn from(child: Sample) -> Self {
        MeterChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for MeterChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        MeterChild::Sample(builder.build())
    }
}
impl From<Script> for MeterChild {
    fn from(child: Script) -> Self {
        MeterChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for MeterChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        MeterChild::Script(builder.build())
    }
}
impl From<Select> for MeterChild {
    fn from(child: Select) -> Self {
        MeterChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for MeterChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        MeterChild::Select(builder.build())
    }
}
impl From<Slot> for MeterChild {
    fn from(child: Slot) -> Self {
        MeterChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for MeterChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        MeterChild::Slot(builder.build())
    }
}
impl From<Small> for MeterChild {
    fn from(child: Small) -> Self {
        MeterChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for MeterChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        MeterChild::Small(builder.build())
    }
}
impl From<Span> for MeterChild {
    fn from(child: Span) -> Self {
        MeterChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for MeterChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        MeterChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for MeterChild {
    fn from(child: StrikeThrough) -> Self {
        MeterChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for MeterChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        MeterChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for MeterChild {
    fn from(child: Strong) -> Self {
        MeterChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for MeterChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        MeterChild::Strong(builder.build())
    }
}
impl From<SubScript> for MeterChild {
    fn from(child: SubScript) -> Self {
        MeterChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for MeterChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        MeterChild::SubScript(builder.build())
    }
}
impl From<SupScript> for MeterChild {
    fn from(child: SupScript) -> Self {
        MeterChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for MeterChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        MeterChild::SupScript(builder.build())
    }
}
impl From<Template> for MeterChild {
    fn from(child: Template) -> Self {
        MeterChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for MeterChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        MeterChild::Template(builder.build())
    }
}
impl From<TextArea> for MeterChild {
    fn from(child: TextArea) -> Self {
        MeterChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for MeterChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        MeterChild::TextArea(builder.build())
    }
}
impl From<Time> for MeterChild {
    fn from(child: Time) -> Self {
        MeterChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for MeterChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        MeterChild::Time(builder.build())
    }
}
impl From<Underline> for MeterChild {
    fn from(child: Underline) -> Self {
        MeterChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for MeterChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        MeterChild::Underline(builder.build())
    }
}
impl From<Variable> for MeterChild {
    fn from(child: Variable) -> Self {
        MeterChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for MeterChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        MeterChild::Variable(builder.build())
    }
}
impl From<Video> for MeterChild {
    fn from(child: Video) -> Self {
        MeterChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for MeterChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        MeterChild::Video(builder.build())
    }
}
impl From<WordBreak> for MeterChild {
    fn from(child: WordBreak) -> Self {
        MeterChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for MeterChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        MeterChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for MeterChild {
    fn from(s: &'static str) -> Self {
        MeterChild::Text(s.into())
    }
}
impl From<String> for MeterChild {
    fn from(s: String) -> Self {
        MeterChild::Text(s.into())
    }
}
impl From<CowStr> for MeterChild {
    fn from(s: CowStr) -> Self {
        MeterChild::Text(s)
    }
}
impl std::fmt::Debug for MeterChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MeterChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Button(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Code(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Data(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Image(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Input(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Label(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Link(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Map(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Object(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Output(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Script(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Select(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Small(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Span(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Template(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Time(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Video(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            MeterChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for MeterChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MeterChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Audio(child) => std::fmt::Display::fmt(child, f),
            MeterChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            MeterChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Bold(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Button(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Cite(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Code(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Custom(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Data(child) => std::fmt::Display::fmt(child, f),
            MeterChild::DataList(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Definition(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Embed(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Image(child) => std::fmt::Display::fmt(child, f),
            MeterChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Input(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Italic(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Label(child) => std::fmt::Display::fmt(child, f),
            MeterChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Link(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Map(child) => std::fmt::Display::fmt(child, f),
            MeterChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Mark(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Meter(child) => std::fmt::Display::fmt(child, f),
            MeterChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Object(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Output(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Picture(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Progress(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Quote(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Sample(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Script(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Select(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Slot(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Small(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Span(child) => std::fmt::Display::fmt(child, f),
            MeterChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Strong(child) => std::fmt::Display::fmt(child, f),
            MeterChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            MeterChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Template(child) => std::fmt::Display::fmt(child, f),
            MeterChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Time(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Underline(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Variable(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Video(child) => std::fmt::Display::fmt(child, f),
            MeterChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            MeterChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
