// 🤖 This file is generated!

use crate::*;
/// The `<em>` element's children.
#[derive(Clone)]
pub enum EmphasisChild {
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
impl From<Abbreviation> for EmphasisChild {
    fn from(child: Abbreviation) -> Self {
        EmphasisChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for EmphasisChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        EmphasisChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for EmphasisChild {
    fn from(child: Anchor) -> Self {
        EmphasisChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for EmphasisChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        EmphasisChild::Anchor(builder.build())
    }
}
impl From<Audio> for EmphasisChild {
    fn from(child: Audio) -> Self {
        EmphasisChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for EmphasisChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        EmphasisChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for EmphasisChild {
    fn from(child: BidirectionalIsolate) -> Self {
        EmphasisChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for EmphasisChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        EmphasisChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for EmphasisChild {
    fn from(child: BidirectionalOverride) -> Self {
        EmphasisChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for EmphasisChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        EmphasisChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for EmphasisChild {
    fn from(child: Bold) -> Self {
        EmphasisChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for EmphasisChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        EmphasisChild::Bold(builder.build())
    }
}
impl From<Button> for EmphasisChild {
    fn from(child: Button) -> Self {
        EmphasisChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for EmphasisChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        EmphasisChild::Button(builder.build())
    }
}
impl From<Canvas> for EmphasisChild {
    fn from(child: Canvas) -> Self {
        EmphasisChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for EmphasisChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        EmphasisChild::Canvas(builder.build())
    }
}
impl From<Cite> for EmphasisChild {
    fn from(child: Cite) -> Self {
        EmphasisChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for EmphasisChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        EmphasisChild::Cite(builder.build())
    }
}
impl From<Code> for EmphasisChild {
    fn from(child: Code) -> Self {
        EmphasisChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for EmphasisChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        EmphasisChild::Code(builder.build())
    }
}
impl From<Custom> for EmphasisChild {
    fn from(child: Custom) -> Self {
        EmphasisChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for EmphasisChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        EmphasisChild::Custom(builder.build())
    }
}
impl From<Data> for EmphasisChild {
    fn from(child: Data) -> Self {
        EmphasisChild::Data(child)
    }
}
impl From<builders::DataBuilder> for EmphasisChild {
    fn from(builder: builders::DataBuilder) -> Self {
        EmphasisChild::Data(builder.build())
    }
}
impl From<DataList> for EmphasisChild {
    fn from(child: DataList) -> Self {
        EmphasisChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for EmphasisChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        EmphasisChild::DataList(builder.build())
    }
}
impl From<Definition> for EmphasisChild {
    fn from(child: Definition) -> Self {
        EmphasisChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for EmphasisChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        EmphasisChild::Definition(builder.build())
    }
}
impl From<Deleted> for EmphasisChild {
    fn from(child: Deleted) -> Self {
        EmphasisChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for EmphasisChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        EmphasisChild::Deleted(builder.build())
    }
}
impl From<Embed> for EmphasisChild {
    fn from(child: Embed) -> Self {
        EmphasisChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for EmphasisChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        EmphasisChild::Embed(builder.build())
    }
}
impl From<Emphasis> for EmphasisChild {
    fn from(child: Emphasis) -> Self {
        EmphasisChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for EmphasisChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        EmphasisChild::Emphasis(builder.build())
    }
}
impl From<Image> for EmphasisChild {
    fn from(child: Image) -> Self {
        EmphasisChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for EmphasisChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        EmphasisChild::Image(builder.build())
    }
}
impl From<InlineFrame> for EmphasisChild {
    fn from(child: InlineFrame) -> Self {
        EmphasisChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for EmphasisChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        EmphasisChild::InlineFrame(builder.build())
    }
}
impl From<Input> for EmphasisChild {
    fn from(child: Input) -> Self {
        EmphasisChild::Input(child)
    }
}
impl From<builders::InputBuilder> for EmphasisChild {
    fn from(builder: builders::InputBuilder) -> Self {
        EmphasisChild::Input(builder.build())
    }
}
impl From<Inserted> for EmphasisChild {
    fn from(child: Inserted) -> Self {
        EmphasisChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for EmphasisChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        EmphasisChild::Inserted(builder.build())
    }
}
impl From<Italic> for EmphasisChild {
    fn from(child: Italic) -> Self {
        EmphasisChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for EmphasisChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        EmphasisChild::Italic(builder.build())
    }
}
impl From<Keyboard> for EmphasisChild {
    fn from(child: Keyboard) -> Self {
        EmphasisChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for EmphasisChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        EmphasisChild::Keyboard(builder.build())
    }
}
impl From<Label> for EmphasisChild {
    fn from(child: Label) -> Self {
        EmphasisChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for EmphasisChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        EmphasisChild::Label(builder.build())
    }
}
impl From<LineBreak> for EmphasisChild {
    fn from(child: LineBreak) -> Self {
        EmphasisChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for EmphasisChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        EmphasisChild::LineBreak(builder.build())
    }
}
impl From<Link> for EmphasisChild {
    fn from(child: Link) -> Self {
        EmphasisChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for EmphasisChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        EmphasisChild::Link(builder.build())
    }
}
impl From<Map> for EmphasisChild {
    fn from(child: Map) -> Self {
        EmphasisChild::Map(child)
    }
}
impl From<builders::MapBuilder> for EmphasisChild {
    fn from(builder: builders::MapBuilder) -> Self {
        EmphasisChild::Map(builder.build())
    }
}
impl From<MapArea> for EmphasisChild {
    fn from(child: MapArea) -> Self {
        EmphasisChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for EmphasisChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        EmphasisChild::MapArea(builder.build())
    }
}
impl From<Mark> for EmphasisChild {
    fn from(child: Mark) -> Self {
        EmphasisChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for EmphasisChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        EmphasisChild::Mark(builder.build())
    }
}
impl From<Metadata> for EmphasisChild {
    fn from(child: Metadata) -> Self {
        EmphasisChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for EmphasisChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        EmphasisChild::Metadata(builder.build())
    }
}
impl From<Meter> for EmphasisChild {
    fn from(child: Meter) -> Self {
        EmphasisChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for EmphasisChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        EmphasisChild::Meter(builder.build())
    }
}
impl From<NoScript> for EmphasisChild {
    fn from(child: NoScript) -> Self {
        EmphasisChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for EmphasisChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        EmphasisChild::NoScript(builder.build())
    }
}
impl From<Object> for EmphasisChild {
    fn from(child: Object) -> Self {
        EmphasisChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for EmphasisChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        EmphasisChild::Object(builder.build())
    }
}
impl From<Output> for EmphasisChild {
    fn from(child: Output) -> Self {
        EmphasisChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for EmphasisChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        EmphasisChild::Output(builder.build())
    }
}
impl From<Picture> for EmphasisChild {
    fn from(child: Picture) -> Self {
        EmphasisChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for EmphasisChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        EmphasisChild::Picture(builder.build())
    }
}
impl From<Progress> for EmphasisChild {
    fn from(child: Progress) -> Self {
        EmphasisChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for EmphasisChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        EmphasisChild::Progress(builder.build())
    }
}
impl From<Quote> for EmphasisChild {
    fn from(child: Quote) -> Self {
        EmphasisChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for EmphasisChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        EmphasisChild::Quote(builder.build())
    }
}
impl From<Ruby> for EmphasisChild {
    fn from(child: Ruby) -> Self {
        EmphasisChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for EmphasisChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        EmphasisChild::Ruby(builder.build())
    }
}
impl From<Sample> for EmphasisChild {
    fn from(child: Sample) -> Self {
        EmphasisChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for EmphasisChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        EmphasisChild::Sample(builder.build())
    }
}
impl From<Script> for EmphasisChild {
    fn from(child: Script) -> Self {
        EmphasisChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for EmphasisChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        EmphasisChild::Script(builder.build())
    }
}
impl From<Select> for EmphasisChild {
    fn from(child: Select) -> Self {
        EmphasisChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for EmphasisChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        EmphasisChild::Select(builder.build())
    }
}
impl From<Slot> for EmphasisChild {
    fn from(child: Slot) -> Self {
        EmphasisChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for EmphasisChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        EmphasisChild::Slot(builder.build())
    }
}
impl From<Small> for EmphasisChild {
    fn from(child: Small) -> Self {
        EmphasisChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for EmphasisChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        EmphasisChild::Small(builder.build())
    }
}
impl From<Span> for EmphasisChild {
    fn from(child: Span) -> Self {
        EmphasisChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for EmphasisChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        EmphasisChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for EmphasisChild {
    fn from(child: StrikeThrough) -> Self {
        EmphasisChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for EmphasisChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        EmphasisChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for EmphasisChild {
    fn from(child: Strong) -> Self {
        EmphasisChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for EmphasisChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        EmphasisChild::Strong(builder.build())
    }
}
impl From<SubScript> for EmphasisChild {
    fn from(child: SubScript) -> Self {
        EmphasisChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for EmphasisChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        EmphasisChild::SubScript(builder.build())
    }
}
impl From<SupScript> for EmphasisChild {
    fn from(child: SupScript) -> Self {
        EmphasisChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for EmphasisChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        EmphasisChild::SupScript(builder.build())
    }
}
impl From<Template> for EmphasisChild {
    fn from(child: Template) -> Self {
        EmphasisChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for EmphasisChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        EmphasisChild::Template(builder.build())
    }
}
impl From<TextArea> for EmphasisChild {
    fn from(child: TextArea) -> Self {
        EmphasisChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for EmphasisChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        EmphasisChild::TextArea(builder.build())
    }
}
impl From<Time> for EmphasisChild {
    fn from(child: Time) -> Self {
        EmphasisChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for EmphasisChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        EmphasisChild::Time(builder.build())
    }
}
impl From<Underline> for EmphasisChild {
    fn from(child: Underline) -> Self {
        EmphasisChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for EmphasisChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        EmphasisChild::Underline(builder.build())
    }
}
impl From<Variable> for EmphasisChild {
    fn from(child: Variable) -> Self {
        EmphasisChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for EmphasisChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        EmphasisChild::Variable(builder.build())
    }
}
impl From<Video> for EmphasisChild {
    fn from(child: Video) -> Self {
        EmphasisChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for EmphasisChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        EmphasisChild::Video(builder.build())
    }
}
impl From<WordBreak> for EmphasisChild {
    fn from(child: WordBreak) -> Self {
        EmphasisChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for EmphasisChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        EmphasisChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for EmphasisChild {
    fn from(s: &'static str) -> Self {
        EmphasisChild::Text(s.into())
    }
}
impl From<String> for EmphasisChild {
    fn from(s: String) -> Self {
        EmphasisChild::Text(s.into())
    }
}
impl From<CowStr> for EmphasisChild {
    fn from(s: CowStr) -> Self {
        EmphasisChild::Text(s)
    }
}
impl std::fmt::Debug for EmphasisChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmphasisChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Button(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Code(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Data(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Image(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Input(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Label(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Link(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Map(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Object(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Output(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Script(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Select(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Small(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Span(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Template(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Time(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Video(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            EmphasisChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for EmphasisChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmphasisChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Audio(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            EmphasisChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            EmphasisChild::Bold(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Button(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Cite(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Code(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Custom(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Data(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::DataList(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Definition(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Embed(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Image(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Input(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Italic(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Label(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Link(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Map(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Mark(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Meter(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Object(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Output(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Picture(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Progress(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Quote(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Sample(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Script(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Select(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Slot(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Small(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Span(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Strong(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Template(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Time(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Underline(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Variable(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Video(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            EmphasisChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
