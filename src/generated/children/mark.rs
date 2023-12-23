// 🤖 This file is generated!

use crate::*;
/// The `<mark>` element's children.
#[derive(Clone)]
pub enum MarkChild {
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
impl From<Abbreviation> for MarkChild {
    fn from(child: Abbreviation) -> Self {
        MarkChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for MarkChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        MarkChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for MarkChild {
    fn from(child: Anchor) -> Self {
        MarkChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for MarkChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        MarkChild::Anchor(builder.build())
    }
}
impl From<Audio> for MarkChild {
    fn from(child: Audio) -> Self {
        MarkChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for MarkChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        MarkChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for MarkChild {
    fn from(child: BidirectionalIsolate) -> Self {
        MarkChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for MarkChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        MarkChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for MarkChild {
    fn from(child: BidirectionalOverride) -> Self {
        MarkChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for MarkChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        MarkChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for MarkChild {
    fn from(child: Bold) -> Self {
        MarkChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for MarkChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        MarkChild::Bold(builder.build())
    }
}
impl From<Button> for MarkChild {
    fn from(child: Button) -> Self {
        MarkChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for MarkChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        MarkChild::Button(builder.build())
    }
}
impl From<Canvas> for MarkChild {
    fn from(child: Canvas) -> Self {
        MarkChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for MarkChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        MarkChild::Canvas(builder.build())
    }
}
impl From<Cite> for MarkChild {
    fn from(child: Cite) -> Self {
        MarkChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for MarkChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        MarkChild::Cite(builder.build())
    }
}
impl From<Code> for MarkChild {
    fn from(child: Code) -> Self {
        MarkChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for MarkChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        MarkChild::Code(builder.build())
    }
}
impl From<Custom> for MarkChild {
    fn from(child: Custom) -> Self {
        MarkChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for MarkChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        MarkChild::Custom(builder.build())
    }
}
impl From<Data> for MarkChild {
    fn from(child: Data) -> Self {
        MarkChild::Data(child)
    }
}
impl From<builders::DataBuilder> for MarkChild {
    fn from(builder: builders::DataBuilder) -> Self {
        MarkChild::Data(builder.build())
    }
}
impl From<DataList> for MarkChild {
    fn from(child: DataList) -> Self {
        MarkChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for MarkChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        MarkChild::DataList(builder.build())
    }
}
impl From<Definition> for MarkChild {
    fn from(child: Definition) -> Self {
        MarkChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for MarkChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        MarkChild::Definition(builder.build())
    }
}
impl From<Deleted> for MarkChild {
    fn from(child: Deleted) -> Self {
        MarkChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for MarkChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        MarkChild::Deleted(builder.build())
    }
}
impl From<Embed> for MarkChild {
    fn from(child: Embed) -> Self {
        MarkChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for MarkChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        MarkChild::Embed(builder.build())
    }
}
impl From<Emphasis> for MarkChild {
    fn from(child: Emphasis) -> Self {
        MarkChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for MarkChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        MarkChild::Emphasis(builder.build())
    }
}
impl From<Image> for MarkChild {
    fn from(child: Image) -> Self {
        MarkChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for MarkChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        MarkChild::Image(builder.build())
    }
}
impl From<InlineFrame> for MarkChild {
    fn from(child: InlineFrame) -> Self {
        MarkChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for MarkChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        MarkChild::InlineFrame(builder.build())
    }
}
impl From<Input> for MarkChild {
    fn from(child: Input) -> Self {
        MarkChild::Input(child)
    }
}
impl From<builders::InputBuilder> for MarkChild {
    fn from(builder: builders::InputBuilder) -> Self {
        MarkChild::Input(builder.build())
    }
}
impl From<Inserted> for MarkChild {
    fn from(child: Inserted) -> Self {
        MarkChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for MarkChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        MarkChild::Inserted(builder.build())
    }
}
impl From<Italic> for MarkChild {
    fn from(child: Italic) -> Self {
        MarkChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for MarkChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        MarkChild::Italic(builder.build())
    }
}
impl From<Keyboard> for MarkChild {
    fn from(child: Keyboard) -> Self {
        MarkChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for MarkChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        MarkChild::Keyboard(builder.build())
    }
}
impl From<Label> for MarkChild {
    fn from(child: Label) -> Self {
        MarkChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for MarkChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        MarkChild::Label(builder.build())
    }
}
impl From<LineBreak> for MarkChild {
    fn from(child: LineBreak) -> Self {
        MarkChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for MarkChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        MarkChild::LineBreak(builder.build())
    }
}
impl From<Link> for MarkChild {
    fn from(child: Link) -> Self {
        MarkChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for MarkChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        MarkChild::Link(builder.build())
    }
}
impl From<Map> for MarkChild {
    fn from(child: Map) -> Self {
        MarkChild::Map(child)
    }
}
impl From<builders::MapBuilder> for MarkChild {
    fn from(builder: builders::MapBuilder) -> Self {
        MarkChild::Map(builder.build())
    }
}
impl From<MapArea> for MarkChild {
    fn from(child: MapArea) -> Self {
        MarkChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for MarkChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        MarkChild::MapArea(builder.build())
    }
}
impl From<Mark> for MarkChild {
    fn from(child: Mark) -> Self {
        MarkChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for MarkChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        MarkChild::Mark(builder.build())
    }
}
impl From<Metadata> for MarkChild {
    fn from(child: Metadata) -> Self {
        MarkChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for MarkChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        MarkChild::Metadata(builder.build())
    }
}
impl From<Meter> for MarkChild {
    fn from(child: Meter) -> Self {
        MarkChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for MarkChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        MarkChild::Meter(builder.build())
    }
}
impl From<NoScript> for MarkChild {
    fn from(child: NoScript) -> Self {
        MarkChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for MarkChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        MarkChild::NoScript(builder.build())
    }
}
impl From<Object> for MarkChild {
    fn from(child: Object) -> Self {
        MarkChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for MarkChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        MarkChild::Object(builder.build())
    }
}
impl From<Output> for MarkChild {
    fn from(child: Output) -> Self {
        MarkChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for MarkChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        MarkChild::Output(builder.build())
    }
}
impl From<Picture> for MarkChild {
    fn from(child: Picture) -> Self {
        MarkChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for MarkChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        MarkChild::Picture(builder.build())
    }
}
impl From<Progress> for MarkChild {
    fn from(child: Progress) -> Self {
        MarkChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for MarkChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        MarkChild::Progress(builder.build())
    }
}
impl From<Quote> for MarkChild {
    fn from(child: Quote) -> Self {
        MarkChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for MarkChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        MarkChild::Quote(builder.build())
    }
}
impl From<Ruby> for MarkChild {
    fn from(child: Ruby) -> Self {
        MarkChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for MarkChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        MarkChild::Ruby(builder.build())
    }
}
impl From<Sample> for MarkChild {
    fn from(child: Sample) -> Self {
        MarkChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for MarkChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        MarkChild::Sample(builder.build())
    }
}
impl From<Script> for MarkChild {
    fn from(child: Script) -> Self {
        MarkChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for MarkChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        MarkChild::Script(builder.build())
    }
}
impl From<Select> for MarkChild {
    fn from(child: Select) -> Self {
        MarkChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for MarkChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        MarkChild::Select(builder.build())
    }
}
impl From<Slot> for MarkChild {
    fn from(child: Slot) -> Self {
        MarkChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for MarkChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        MarkChild::Slot(builder.build())
    }
}
impl From<Small> for MarkChild {
    fn from(child: Small) -> Self {
        MarkChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for MarkChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        MarkChild::Small(builder.build())
    }
}
impl From<Span> for MarkChild {
    fn from(child: Span) -> Self {
        MarkChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for MarkChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        MarkChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for MarkChild {
    fn from(child: StrikeThrough) -> Self {
        MarkChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for MarkChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        MarkChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for MarkChild {
    fn from(child: Strong) -> Self {
        MarkChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for MarkChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        MarkChild::Strong(builder.build())
    }
}
impl From<SubScript> for MarkChild {
    fn from(child: SubScript) -> Self {
        MarkChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for MarkChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        MarkChild::SubScript(builder.build())
    }
}
impl From<SupScript> for MarkChild {
    fn from(child: SupScript) -> Self {
        MarkChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for MarkChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        MarkChild::SupScript(builder.build())
    }
}
impl From<Template> for MarkChild {
    fn from(child: Template) -> Self {
        MarkChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for MarkChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        MarkChild::Template(builder.build())
    }
}
impl From<TextArea> for MarkChild {
    fn from(child: TextArea) -> Self {
        MarkChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for MarkChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        MarkChild::TextArea(builder.build())
    }
}
impl From<Time> for MarkChild {
    fn from(child: Time) -> Self {
        MarkChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for MarkChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        MarkChild::Time(builder.build())
    }
}
impl From<Underline> for MarkChild {
    fn from(child: Underline) -> Self {
        MarkChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for MarkChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        MarkChild::Underline(builder.build())
    }
}
impl From<Variable> for MarkChild {
    fn from(child: Variable) -> Self {
        MarkChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for MarkChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        MarkChild::Variable(builder.build())
    }
}
impl From<Video> for MarkChild {
    fn from(child: Video) -> Self {
        MarkChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for MarkChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        MarkChild::Video(builder.build())
    }
}
impl From<WordBreak> for MarkChild {
    fn from(child: WordBreak) -> Self {
        MarkChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for MarkChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        MarkChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for MarkChild {
    fn from(s: &'static str) -> Self {
        MarkChild::Text(s.into())
    }
}
impl From<String> for MarkChild {
    fn from(s: String) -> Self {
        MarkChild::Text(s.into())
    }
}
impl From<CowStr> for MarkChild {
    fn from(s: CowStr) -> Self {
        MarkChild::Text(s)
    }
}
impl std::fmt::Debug for MarkChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarkChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Button(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Code(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Data(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Image(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Input(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Label(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Link(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Map(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Object(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Output(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Script(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Select(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Small(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Span(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Template(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Time(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Video(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            MarkChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for MarkChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarkChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Audio(child) => std::fmt::Display::fmt(child, f),
            MarkChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            MarkChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Bold(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Button(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Cite(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Code(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Custom(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Data(child) => std::fmt::Display::fmt(child, f),
            MarkChild::DataList(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Definition(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Embed(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Image(child) => std::fmt::Display::fmt(child, f),
            MarkChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Input(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Italic(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Label(child) => std::fmt::Display::fmt(child, f),
            MarkChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Link(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Map(child) => std::fmt::Display::fmt(child, f),
            MarkChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Mark(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Meter(child) => std::fmt::Display::fmt(child, f),
            MarkChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Object(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Output(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Picture(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Progress(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Quote(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Sample(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Script(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Select(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Slot(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Small(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Span(child) => std::fmt::Display::fmt(child, f),
            MarkChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Strong(child) => std::fmt::Display::fmt(child, f),
            MarkChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            MarkChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Template(child) => std::fmt::Display::fmt(child, f),
            MarkChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Time(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Underline(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Variable(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Video(child) => std::fmt::Display::fmt(child, f),
            MarkChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            MarkChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
