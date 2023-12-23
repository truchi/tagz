// 🤖 This file is generated!

use crate::*;
/// The `<h5>` element's children.
#[derive(Clone)]
pub enum Heading5Child {
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
impl From<Abbreviation> for Heading5Child {
    fn from(child: Abbreviation) -> Self {
        Heading5Child::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for Heading5Child {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        Heading5Child::Abbreviation(builder.build())
    }
}
impl From<Anchor> for Heading5Child {
    fn from(child: Anchor) -> Self {
        Heading5Child::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for Heading5Child {
    fn from(builder: builders::AnchorBuilder) -> Self {
        Heading5Child::Anchor(builder.build())
    }
}
impl From<Audio> for Heading5Child {
    fn from(child: Audio) -> Self {
        Heading5Child::Audio(child)
    }
}
impl From<builders::AudioBuilder> for Heading5Child {
    fn from(builder: builders::AudioBuilder) -> Self {
        Heading5Child::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for Heading5Child {
    fn from(child: BidirectionalIsolate) -> Self {
        Heading5Child::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for Heading5Child {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        Heading5Child::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for Heading5Child {
    fn from(child: BidirectionalOverride) -> Self {
        Heading5Child::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for Heading5Child {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        Heading5Child::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for Heading5Child {
    fn from(child: Bold) -> Self {
        Heading5Child::Bold(child)
    }
}
impl From<builders::BoldBuilder> for Heading5Child {
    fn from(builder: builders::BoldBuilder) -> Self {
        Heading5Child::Bold(builder.build())
    }
}
impl From<Button> for Heading5Child {
    fn from(child: Button) -> Self {
        Heading5Child::Button(child)
    }
}
impl From<builders::ButtonBuilder> for Heading5Child {
    fn from(builder: builders::ButtonBuilder) -> Self {
        Heading5Child::Button(builder.build())
    }
}
impl From<Canvas> for Heading5Child {
    fn from(child: Canvas) -> Self {
        Heading5Child::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for Heading5Child {
    fn from(builder: builders::CanvasBuilder) -> Self {
        Heading5Child::Canvas(builder.build())
    }
}
impl From<Cite> for Heading5Child {
    fn from(child: Cite) -> Self {
        Heading5Child::Cite(child)
    }
}
impl From<builders::CiteBuilder> for Heading5Child {
    fn from(builder: builders::CiteBuilder) -> Self {
        Heading5Child::Cite(builder.build())
    }
}
impl From<Code> for Heading5Child {
    fn from(child: Code) -> Self {
        Heading5Child::Code(child)
    }
}
impl From<builders::CodeBuilder> for Heading5Child {
    fn from(builder: builders::CodeBuilder) -> Self {
        Heading5Child::Code(builder.build())
    }
}
impl From<Custom> for Heading5Child {
    fn from(child: Custom) -> Self {
        Heading5Child::Custom(child)
    }
}
impl From<builders::CustomBuilder> for Heading5Child {
    fn from(builder: builders::CustomBuilder) -> Self {
        Heading5Child::Custom(builder.build())
    }
}
impl From<Data> for Heading5Child {
    fn from(child: Data) -> Self {
        Heading5Child::Data(child)
    }
}
impl From<builders::DataBuilder> for Heading5Child {
    fn from(builder: builders::DataBuilder) -> Self {
        Heading5Child::Data(builder.build())
    }
}
impl From<DataList> for Heading5Child {
    fn from(child: DataList) -> Self {
        Heading5Child::DataList(child)
    }
}
impl From<builders::DataListBuilder> for Heading5Child {
    fn from(builder: builders::DataListBuilder) -> Self {
        Heading5Child::DataList(builder.build())
    }
}
impl From<Definition> for Heading5Child {
    fn from(child: Definition) -> Self {
        Heading5Child::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for Heading5Child {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        Heading5Child::Definition(builder.build())
    }
}
impl From<Deleted> for Heading5Child {
    fn from(child: Deleted) -> Self {
        Heading5Child::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for Heading5Child {
    fn from(builder: builders::DeletedBuilder) -> Self {
        Heading5Child::Deleted(builder.build())
    }
}
impl From<Embed> for Heading5Child {
    fn from(child: Embed) -> Self {
        Heading5Child::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for Heading5Child {
    fn from(builder: builders::EmbedBuilder) -> Self {
        Heading5Child::Embed(builder.build())
    }
}
impl From<Emphasis> for Heading5Child {
    fn from(child: Emphasis) -> Self {
        Heading5Child::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for Heading5Child {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        Heading5Child::Emphasis(builder.build())
    }
}
impl From<Image> for Heading5Child {
    fn from(child: Image) -> Self {
        Heading5Child::Image(child)
    }
}
impl From<builders::ImageBuilder> for Heading5Child {
    fn from(builder: builders::ImageBuilder) -> Self {
        Heading5Child::Image(builder.build())
    }
}
impl From<InlineFrame> for Heading5Child {
    fn from(child: InlineFrame) -> Self {
        Heading5Child::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for Heading5Child {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        Heading5Child::InlineFrame(builder.build())
    }
}
impl From<Input> for Heading5Child {
    fn from(child: Input) -> Self {
        Heading5Child::Input(child)
    }
}
impl From<builders::InputBuilder> for Heading5Child {
    fn from(builder: builders::InputBuilder) -> Self {
        Heading5Child::Input(builder.build())
    }
}
impl From<Inserted> for Heading5Child {
    fn from(child: Inserted) -> Self {
        Heading5Child::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for Heading5Child {
    fn from(builder: builders::InsertedBuilder) -> Self {
        Heading5Child::Inserted(builder.build())
    }
}
impl From<Italic> for Heading5Child {
    fn from(child: Italic) -> Self {
        Heading5Child::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for Heading5Child {
    fn from(builder: builders::ItalicBuilder) -> Self {
        Heading5Child::Italic(builder.build())
    }
}
impl From<Keyboard> for Heading5Child {
    fn from(child: Keyboard) -> Self {
        Heading5Child::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for Heading5Child {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        Heading5Child::Keyboard(builder.build())
    }
}
impl From<Label> for Heading5Child {
    fn from(child: Label) -> Self {
        Heading5Child::Label(child)
    }
}
impl From<builders::LabelBuilder> for Heading5Child {
    fn from(builder: builders::LabelBuilder) -> Self {
        Heading5Child::Label(builder.build())
    }
}
impl From<LineBreak> for Heading5Child {
    fn from(child: LineBreak) -> Self {
        Heading5Child::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for Heading5Child {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        Heading5Child::LineBreak(builder.build())
    }
}
impl From<Link> for Heading5Child {
    fn from(child: Link) -> Self {
        Heading5Child::Link(child)
    }
}
impl From<builders::LinkBuilder> for Heading5Child {
    fn from(builder: builders::LinkBuilder) -> Self {
        Heading5Child::Link(builder.build())
    }
}
impl From<Map> for Heading5Child {
    fn from(child: Map) -> Self {
        Heading5Child::Map(child)
    }
}
impl From<builders::MapBuilder> for Heading5Child {
    fn from(builder: builders::MapBuilder) -> Self {
        Heading5Child::Map(builder.build())
    }
}
impl From<MapArea> for Heading5Child {
    fn from(child: MapArea) -> Self {
        Heading5Child::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for Heading5Child {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        Heading5Child::MapArea(builder.build())
    }
}
impl From<Mark> for Heading5Child {
    fn from(child: Mark) -> Self {
        Heading5Child::Mark(child)
    }
}
impl From<builders::MarkBuilder> for Heading5Child {
    fn from(builder: builders::MarkBuilder) -> Self {
        Heading5Child::Mark(builder.build())
    }
}
impl From<Metadata> for Heading5Child {
    fn from(child: Metadata) -> Self {
        Heading5Child::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for Heading5Child {
    fn from(builder: builders::MetadataBuilder) -> Self {
        Heading5Child::Metadata(builder.build())
    }
}
impl From<Meter> for Heading5Child {
    fn from(child: Meter) -> Self {
        Heading5Child::Meter(child)
    }
}
impl From<builders::MeterBuilder> for Heading5Child {
    fn from(builder: builders::MeterBuilder) -> Self {
        Heading5Child::Meter(builder.build())
    }
}
impl From<NoScript> for Heading5Child {
    fn from(child: NoScript) -> Self {
        Heading5Child::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for Heading5Child {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        Heading5Child::NoScript(builder.build())
    }
}
impl From<Object> for Heading5Child {
    fn from(child: Object) -> Self {
        Heading5Child::Object(child)
    }
}
impl From<builders::ObjectBuilder> for Heading5Child {
    fn from(builder: builders::ObjectBuilder) -> Self {
        Heading5Child::Object(builder.build())
    }
}
impl From<Output> for Heading5Child {
    fn from(child: Output) -> Self {
        Heading5Child::Output(child)
    }
}
impl From<builders::OutputBuilder> for Heading5Child {
    fn from(builder: builders::OutputBuilder) -> Self {
        Heading5Child::Output(builder.build())
    }
}
impl From<Picture> for Heading5Child {
    fn from(child: Picture) -> Self {
        Heading5Child::Picture(child)
    }
}
impl From<builders::PictureBuilder> for Heading5Child {
    fn from(builder: builders::PictureBuilder) -> Self {
        Heading5Child::Picture(builder.build())
    }
}
impl From<Progress> for Heading5Child {
    fn from(child: Progress) -> Self {
        Heading5Child::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for Heading5Child {
    fn from(builder: builders::ProgressBuilder) -> Self {
        Heading5Child::Progress(builder.build())
    }
}
impl From<Quote> for Heading5Child {
    fn from(child: Quote) -> Self {
        Heading5Child::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for Heading5Child {
    fn from(builder: builders::QuoteBuilder) -> Self {
        Heading5Child::Quote(builder.build())
    }
}
impl From<Ruby> for Heading5Child {
    fn from(child: Ruby) -> Self {
        Heading5Child::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for Heading5Child {
    fn from(builder: builders::RubyBuilder) -> Self {
        Heading5Child::Ruby(builder.build())
    }
}
impl From<Sample> for Heading5Child {
    fn from(child: Sample) -> Self {
        Heading5Child::Sample(child)
    }
}
impl From<builders::SampleBuilder> for Heading5Child {
    fn from(builder: builders::SampleBuilder) -> Self {
        Heading5Child::Sample(builder.build())
    }
}
impl From<Script> for Heading5Child {
    fn from(child: Script) -> Self {
        Heading5Child::Script(child)
    }
}
impl From<builders::ScriptBuilder> for Heading5Child {
    fn from(builder: builders::ScriptBuilder) -> Self {
        Heading5Child::Script(builder.build())
    }
}
impl From<Select> for Heading5Child {
    fn from(child: Select) -> Self {
        Heading5Child::Select(child)
    }
}
impl From<builders::SelectBuilder> for Heading5Child {
    fn from(builder: builders::SelectBuilder) -> Self {
        Heading5Child::Select(builder.build())
    }
}
impl From<Slot> for Heading5Child {
    fn from(child: Slot) -> Self {
        Heading5Child::Slot(child)
    }
}
impl From<builders::SlotBuilder> for Heading5Child {
    fn from(builder: builders::SlotBuilder) -> Self {
        Heading5Child::Slot(builder.build())
    }
}
impl From<Small> for Heading5Child {
    fn from(child: Small) -> Self {
        Heading5Child::Small(child)
    }
}
impl From<builders::SmallBuilder> for Heading5Child {
    fn from(builder: builders::SmallBuilder) -> Self {
        Heading5Child::Small(builder.build())
    }
}
impl From<Span> for Heading5Child {
    fn from(child: Span) -> Self {
        Heading5Child::Span(child)
    }
}
impl From<builders::SpanBuilder> for Heading5Child {
    fn from(builder: builders::SpanBuilder) -> Self {
        Heading5Child::Span(builder.build())
    }
}
impl From<StrikeThrough> for Heading5Child {
    fn from(child: StrikeThrough) -> Self {
        Heading5Child::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for Heading5Child {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        Heading5Child::StrikeThrough(builder.build())
    }
}
impl From<Strong> for Heading5Child {
    fn from(child: Strong) -> Self {
        Heading5Child::Strong(child)
    }
}
impl From<builders::StrongBuilder> for Heading5Child {
    fn from(builder: builders::StrongBuilder) -> Self {
        Heading5Child::Strong(builder.build())
    }
}
impl From<SubScript> for Heading5Child {
    fn from(child: SubScript) -> Self {
        Heading5Child::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for Heading5Child {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        Heading5Child::SubScript(builder.build())
    }
}
impl From<SupScript> for Heading5Child {
    fn from(child: SupScript) -> Self {
        Heading5Child::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for Heading5Child {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        Heading5Child::SupScript(builder.build())
    }
}
impl From<Template> for Heading5Child {
    fn from(child: Template) -> Self {
        Heading5Child::Template(child)
    }
}
impl From<builders::TemplateBuilder> for Heading5Child {
    fn from(builder: builders::TemplateBuilder) -> Self {
        Heading5Child::Template(builder.build())
    }
}
impl From<TextArea> for Heading5Child {
    fn from(child: TextArea) -> Self {
        Heading5Child::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for Heading5Child {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        Heading5Child::TextArea(builder.build())
    }
}
impl From<Time> for Heading5Child {
    fn from(child: Time) -> Self {
        Heading5Child::Time(child)
    }
}
impl From<builders::TimeBuilder> for Heading5Child {
    fn from(builder: builders::TimeBuilder) -> Self {
        Heading5Child::Time(builder.build())
    }
}
impl From<Underline> for Heading5Child {
    fn from(child: Underline) -> Self {
        Heading5Child::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for Heading5Child {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        Heading5Child::Underline(builder.build())
    }
}
impl From<Variable> for Heading5Child {
    fn from(child: Variable) -> Self {
        Heading5Child::Variable(child)
    }
}
impl From<builders::VariableBuilder> for Heading5Child {
    fn from(builder: builders::VariableBuilder) -> Self {
        Heading5Child::Variable(builder.build())
    }
}
impl From<Video> for Heading5Child {
    fn from(child: Video) -> Self {
        Heading5Child::Video(child)
    }
}
impl From<builders::VideoBuilder> for Heading5Child {
    fn from(builder: builders::VideoBuilder) -> Self {
        Heading5Child::Video(builder.build())
    }
}
impl From<WordBreak> for Heading5Child {
    fn from(child: WordBreak) -> Self {
        Heading5Child::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for Heading5Child {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        Heading5Child::WordBreak(builder.build())
    }
}
impl From<&'static str> for Heading5Child {
    fn from(s: &'static str) -> Self {
        Heading5Child::Text(s.into())
    }
}
impl From<String> for Heading5Child {
    fn from(s: String) -> Self {
        Heading5Child::Text(s.into())
    }
}
impl From<CowStr> for Heading5Child {
    fn from(s: CowStr) -> Self {
        Heading5Child::Text(s)
    }
}
impl std::fmt::Debug for Heading5Child {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Heading5Child::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Anchor(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Audio(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Bold(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Button(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Canvas(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Cite(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Code(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Custom(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Data(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::DataList(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Definition(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Deleted(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Embed(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Image(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Input(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Inserted(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Italic(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Label(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Link(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Map(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::MapArea(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Mark(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Metadata(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Meter(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::NoScript(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Object(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Output(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Picture(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Progress(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Quote(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Ruby(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Sample(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Script(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Select(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Slot(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Small(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Span(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Strong(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::SubScript(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::SupScript(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Template(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::TextArea(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Time(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Underline(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Variable(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Video(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            Heading5Child::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for Heading5Child {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Heading5Child::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Anchor(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Audio(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            Heading5Child::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            Heading5Child::Bold(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Button(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Canvas(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Cite(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Code(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Custom(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Data(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::DataList(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Definition(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Deleted(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Embed(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Emphasis(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Image(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Input(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Inserted(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Italic(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Keyboard(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Label(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::LineBreak(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Link(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Map(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::MapArea(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Mark(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Metadata(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Meter(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::NoScript(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Object(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Output(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Picture(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Progress(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Quote(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Ruby(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Sample(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Script(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Select(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Slot(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Small(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Span(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Strong(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::SubScript(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::SupScript(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Template(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::TextArea(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Time(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Underline(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Variable(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Video(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::WordBreak(child) => std::fmt::Display::fmt(child, f),
            Heading5Child::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
