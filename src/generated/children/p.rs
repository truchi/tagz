// 🤖 This file is generated!

use crate::*;
/// The `<p>` element's children.
#[derive(Clone)]
pub enum ParagraphChild {
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
impl From<Abbreviation> for ParagraphChild {
    fn from(child: Abbreviation) -> Self {
        ParagraphChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for ParagraphChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        ParagraphChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for ParagraphChild {
    fn from(child: Anchor) -> Self {
        ParagraphChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for ParagraphChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        ParagraphChild::Anchor(builder.build())
    }
}
impl From<Audio> for ParagraphChild {
    fn from(child: Audio) -> Self {
        ParagraphChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for ParagraphChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        ParagraphChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for ParagraphChild {
    fn from(child: BidirectionalIsolate) -> Self {
        ParagraphChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for ParagraphChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        ParagraphChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for ParagraphChild {
    fn from(child: BidirectionalOverride) -> Self {
        ParagraphChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for ParagraphChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        ParagraphChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for ParagraphChild {
    fn from(child: Bold) -> Self {
        ParagraphChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for ParagraphChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        ParagraphChild::Bold(builder.build())
    }
}
impl From<Button> for ParagraphChild {
    fn from(child: Button) -> Self {
        ParagraphChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for ParagraphChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        ParagraphChild::Button(builder.build())
    }
}
impl From<Canvas> for ParagraphChild {
    fn from(child: Canvas) -> Self {
        ParagraphChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for ParagraphChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        ParagraphChild::Canvas(builder.build())
    }
}
impl From<Cite> for ParagraphChild {
    fn from(child: Cite) -> Self {
        ParagraphChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for ParagraphChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        ParagraphChild::Cite(builder.build())
    }
}
impl From<Code> for ParagraphChild {
    fn from(child: Code) -> Self {
        ParagraphChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for ParagraphChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        ParagraphChild::Code(builder.build())
    }
}
impl From<Custom> for ParagraphChild {
    fn from(child: Custom) -> Self {
        ParagraphChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for ParagraphChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        ParagraphChild::Custom(builder.build())
    }
}
impl From<Data> for ParagraphChild {
    fn from(child: Data) -> Self {
        ParagraphChild::Data(child)
    }
}
impl From<builders::DataBuilder> for ParagraphChild {
    fn from(builder: builders::DataBuilder) -> Self {
        ParagraphChild::Data(builder.build())
    }
}
impl From<DataList> for ParagraphChild {
    fn from(child: DataList) -> Self {
        ParagraphChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for ParagraphChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        ParagraphChild::DataList(builder.build())
    }
}
impl From<Definition> for ParagraphChild {
    fn from(child: Definition) -> Self {
        ParagraphChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for ParagraphChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        ParagraphChild::Definition(builder.build())
    }
}
impl From<Deleted> for ParagraphChild {
    fn from(child: Deleted) -> Self {
        ParagraphChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for ParagraphChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        ParagraphChild::Deleted(builder.build())
    }
}
impl From<Embed> for ParagraphChild {
    fn from(child: Embed) -> Self {
        ParagraphChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for ParagraphChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        ParagraphChild::Embed(builder.build())
    }
}
impl From<Emphasis> for ParagraphChild {
    fn from(child: Emphasis) -> Self {
        ParagraphChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for ParagraphChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        ParagraphChild::Emphasis(builder.build())
    }
}
impl From<Image> for ParagraphChild {
    fn from(child: Image) -> Self {
        ParagraphChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for ParagraphChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        ParagraphChild::Image(builder.build())
    }
}
impl From<InlineFrame> for ParagraphChild {
    fn from(child: InlineFrame) -> Self {
        ParagraphChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for ParagraphChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        ParagraphChild::InlineFrame(builder.build())
    }
}
impl From<Input> for ParagraphChild {
    fn from(child: Input) -> Self {
        ParagraphChild::Input(child)
    }
}
impl From<builders::InputBuilder> for ParagraphChild {
    fn from(builder: builders::InputBuilder) -> Self {
        ParagraphChild::Input(builder.build())
    }
}
impl From<Inserted> for ParagraphChild {
    fn from(child: Inserted) -> Self {
        ParagraphChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for ParagraphChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        ParagraphChild::Inserted(builder.build())
    }
}
impl From<Italic> for ParagraphChild {
    fn from(child: Italic) -> Self {
        ParagraphChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for ParagraphChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        ParagraphChild::Italic(builder.build())
    }
}
impl From<Keyboard> for ParagraphChild {
    fn from(child: Keyboard) -> Self {
        ParagraphChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for ParagraphChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        ParagraphChild::Keyboard(builder.build())
    }
}
impl From<Label> for ParagraphChild {
    fn from(child: Label) -> Self {
        ParagraphChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for ParagraphChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        ParagraphChild::Label(builder.build())
    }
}
impl From<LineBreak> for ParagraphChild {
    fn from(child: LineBreak) -> Self {
        ParagraphChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for ParagraphChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        ParagraphChild::LineBreak(builder.build())
    }
}
impl From<Link> for ParagraphChild {
    fn from(child: Link) -> Self {
        ParagraphChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for ParagraphChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        ParagraphChild::Link(builder.build())
    }
}
impl From<Map> for ParagraphChild {
    fn from(child: Map) -> Self {
        ParagraphChild::Map(child)
    }
}
impl From<builders::MapBuilder> for ParagraphChild {
    fn from(builder: builders::MapBuilder) -> Self {
        ParagraphChild::Map(builder.build())
    }
}
impl From<MapArea> for ParagraphChild {
    fn from(child: MapArea) -> Self {
        ParagraphChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for ParagraphChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        ParagraphChild::MapArea(builder.build())
    }
}
impl From<Mark> for ParagraphChild {
    fn from(child: Mark) -> Self {
        ParagraphChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for ParagraphChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        ParagraphChild::Mark(builder.build())
    }
}
impl From<Metadata> for ParagraphChild {
    fn from(child: Metadata) -> Self {
        ParagraphChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for ParagraphChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        ParagraphChild::Metadata(builder.build())
    }
}
impl From<Meter> for ParagraphChild {
    fn from(child: Meter) -> Self {
        ParagraphChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for ParagraphChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        ParagraphChild::Meter(builder.build())
    }
}
impl From<NoScript> for ParagraphChild {
    fn from(child: NoScript) -> Self {
        ParagraphChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for ParagraphChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        ParagraphChild::NoScript(builder.build())
    }
}
impl From<Object> for ParagraphChild {
    fn from(child: Object) -> Self {
        ParagraphChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for ParagraphChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        ParagraphChild::Object(builder.build())
    }
}
impl From<Output> for ParagraphChild {
    fn from(child: Output) -> Self {
        ParagraphChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for ParagraphChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        ParagraphChild::Output(builder.build())
    }
}
impl From<Picture> for ParagraphChild {
    fn from(child: Picture) -> Self {
        ParagraphChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for ParagraphChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        ParagraphChild::Picture(builder.build())
    }
}
impl From<Progress> for ParagraphChild {
    fn from(child: Progress) -> Self {
        ParagraphChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for ParagraphChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        ParagraphChild::Progress(builder.build())
    }
}
impl From<Quote> for ParagraphChild {
    fn from(child: Quote) -> Self {
        ParagraphChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for ParagraphChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        ParagraphChild::Quote(builder.build())
    }
}
impl From<Ruby> for ParagraphChild {
    fn from(child: Ruby) -> Self {
        ParagraphChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for ParagraphChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        ParagraphChild::Ruby(builder.build())
    }
}
impl From<Sample> for ParagraphChild {
    fn from(child: Sample) -> Self {
        ParagraphChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for ParagraphChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        ParagraphChild::Sample(builder.build())
    }
}
impl From<Script> for ParagraphChild {
    fn from(child: Script) -> Self {
        ParagraphChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for ParagraphChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        ParagraphChild::Script(builder.build())
    }
}
impl From<Select> for ParagraphChild {
    fn from(child: Select) -> Self {
        ParagraphChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for ParagraphChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        ParagraphChild::Select(builder.build())
    }
}
impl From<Slot> for ParagraphChild {
    fn from(child: Slot) -> Self {
        ParagraphChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for ParagraphChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        ParagraphChild::Slot(builder.build())
    }
}
impl From<Small> for ParagraphChild {
    fn from(child: Small) -> Self {
        ParagraphChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for ParagraphChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        ParagraphChild::Small(builder.build())
    }
}
impl From<Span> for ParagraphChild {
    fn from(child: Span) -> Self {
        ParagraphChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for ParagraphChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        ParagraphChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for ParagraphChild {
    fn from(child: StrikeThrough) -> Self {
        ParagraphChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for ParagraphChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        ParagraphChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for ParagraphChild {
    fn from(child: Strong) -> Self {
        ParagraphChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for ParagraphChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        ParagraphChild::Strong(builder.build())
    }
}
impl From<SubScript> for ParagraphChild {
    fn from(child: SubScript) -> Self {
        ParagraphChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for ParagraphChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        ParagraphChild::SubScript(builder.build())
    }
}
impl From<SupScript> for ParagraphChild {
    fn from(child: SupScript) -> Self {
        ParagraphChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for ParagraphChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        ParagraphChild::SupScript(builder.build())
    }
}
impl From<Template> for ParagraphChild {
    fn from(child: Template) -> Self {
        ParagraphChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for ParagraphChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        ParagraphChild::Template(builder.build())
    }
}
impl From<TextArea> for ParagraphChild {
    fn from(child: TextArea) -> Self {
        ParagraphChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for ParagraphChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        ParagraphChild::TextArea(builder.build())
    }
}
impl From<Time> for ParagraphChild {
    fn from(child: Time) -> Self {
        ParagraphChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for ParagraphChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        ParagraphChild::Time(builder.build())
    }
}
impl From<Underline> for ParagraphChild {
    fn from(child: Underline) -> Self {
        ParagraphChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for ParagraphChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        ParagraphChild::Underline(builder.build())
    }
}
impl From<Variable> for ParagraphChild {
    fn from(child: Variable) -> Self {
        ParagraphChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for ParagraphChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        ParagraphChild::Variable(builder.build())
    }
}
impl From<Video> for ParagraphChild {
    fn from(child: Video) -> Self {
        ParagraphChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for ParagraphChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        ParagraphChild::Video(builder.build())
    }
}
impl From<WordBreak> for ParagraphChild {
    fn from(child: WordBreak) -> Self {
        ParagraphChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for ParagraphChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        ParagraphChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for ParagraphChild {
    fn from(s: &'static str) -> Self {
        ParagraphChild::Text(s.into())
    }
}
impl From<String> for ParagraphChild {
    fn from(s: String) -> Self {
        ParagraphChild::Text(s.into())
    }
}
impl From<CowStr> for ParagraphChild {
    fn from(s: CowStr) -> Self {
        ParagraphChild::Text(s)
    }
}
impl std::fmt::Debug for ParagraphChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::BidirectionalOverride(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            ParagraphChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Button(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Code(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Data(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Image(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Input(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Label(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Link(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Map(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Object(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Output(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Script(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Select(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Small(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Span(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Template(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Time(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Video(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            ParagraphChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for ParagraphChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Audio(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            ParagraphChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            ParagraphChild::Bold(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Button(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Cite(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Code(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Custom(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Data(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::DataList(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Definition(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Embed(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Image(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Input(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Italic(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Label(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Link(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Map(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Mark(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Meter(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Object(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Output(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Picture(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Progress(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Quote(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Sample(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Script(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Select(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Slot(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Small(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Span(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Strong(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Template(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Time(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Underline(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Variable(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Video(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            ParagraphChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
