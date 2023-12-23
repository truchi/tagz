// 🤖 This file is generated!

use crate::*;
/// The `<dfn>` element's children.
#[derive(Clone)]
pub enum DefinitionChild {
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
impl From<Abbreviation> for DefinitionChild {
    fn from(child: Abbreviation) -> Self {
        DefinitionChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for DefinitionChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        DefinitionChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for DefinitionChild {
    fn from(child: Anchor) -> Self {
        DefinitionChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for DefinitionChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        DefinitionChild::Anchor(builder.build())
    }
}
impl From<Audio> for DefinitionChild {
    fn from(child: Audio) -> Self {
        DefinitionChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for DefinitionChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        DefinitionChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for DefinitionChild {
    fn from(child: BidirectionalIsolate) -> Self {
        DefinitionChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for DefinitionChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        DefinitionChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for DefinitionChild {
    fn from(child: BidirectionalOverride) -> Self {
        DefinitionChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for DefinitionChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        DefinitionChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for DefinitionChild {
    fn from(child: Bold) -> Self {
        DefinitionChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for DefinitionChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        DefinitionChild::Bold(builder.build())
    }
}
impl From<Button> for DefinitionChild {
    fn from(child: Button) -> Self {
        DefinitionChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for DefinitionChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        DefinitionChild::Button(builder.build())
    }
}
impl From<Canvas> for DefinitionChild {
    fn from(child: Canvas) -> Self {
        DefinitionChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for DefinitionChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        DefinitionChild::Canvas(builder.build())
    }
}
impl From<Cite> for DefinitionChild {
    fn from(child: Cite) -> Self {
        DefinitionChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for DefinitionChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        DefinitionChild::Cite(builder.build())
    }
}
impl From<Code> for DefinitionChild {
    fn from(child: Code) -> Self {
        DefinitionChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for DefinitionChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        DefinitionChild::Code(builder.build())
    }
}
impl From<Custom> for DefinitionChild {
    fn from(child: Custom) -> Self {
        DefinitionChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for DefinitionChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        DefinitionChild::Custom(builder.build())
    }
}
impl From<Data> for DefinitionChild {
    fn from(child: Data) -> Self {
        DefinitionChild::Data(child)
    }
}
impl From<builders::DataBuilder> for DefinitionChild {
    fn from(builder: builders::DataBuilder) -> Self {
        DefinitionChild::Data(builder.build())
    }
}
impl From<DataList> for DefinitionChild {
    fn from(child: DataList) -> Self {
        DefinitionChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for DefinitionChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        DefinitionChild::DataList(builder.build())
    }
}
impl From<Definition> for DefinitionChild {
    fn from(child: Definition) -> Self {
        DefinitionChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for DefinitionChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        DefinitionChild::Definition(builder.build())
    }
}
impl From<Deleted> for DefinitionChild {
    fn from(child: Deleted) -> Self {
        DefinitionChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for DefinitionChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        DefinitionChild::Deleted(builder.build())
    }
}
impl From<Embed> for DefinitionChild {
    fn from(child: Embed) -> Self {
        DefinitionChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for DefinitionChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        DefinitionChild::Embed(builder.build())
    }
}
impl From<Emphasis> for DefinitionChild {
    fn from(child: Emphasis) -> Self {
        DefinitionChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for DefinitionChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        DefinitionChild::Emphasis(builder.build())
    }
}
impl From<Image> for DefinitionChild {
    fn from(child: Image) -> Self {
        DefinitionChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for DefinitionChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        DefinitionChild::Image(builder.build())
    }
}
impl From<InlineFrame> for DefinitionChild {
    fn from(child: InlineFrame) -> Self {
        DefinitionChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for DefinitionChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        DefinitionChild::InlineFrame(builder.build())
    }
}
impl From<Input> for DefinitionChild {
    fn from(child: Input) -> Self {
        DefinitionChild::Input(child)
    }
}
impl From<builders::InputBuilder> for DefinitionChild {
    fn from(builder: builders::InputBuilder) -> Self {
        DefinitionChild::Input(builder.build())
    }
}
impl From<Inserted> for DefinitionChild {
    fn from(child: Inserted) -> Self {
        DefinitionChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for DefinitionChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        DefinitionChild::Inserted(builder.build())
    }
}
impl From<Italic> for DefinitionChild {
    fn from(child: Italic) -> Self {
        DefinitionChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for DefinitionChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        DefinitionChild::Italic(builder.build())
    }
}
impl From<Keyboard> for DefinitionChild {
    fn from(child: Keyboard) -> Self {
        DefinitionChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for DefinitionChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        DefinitionChild::Keyboard(builder.build())
    }
}
impl From<Label> for DefinitionChild {
    fn from(child: Label) -> Self {
        DefinitionChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for DefinitionChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        DefinitionChild::Label(builder.build())
    }
}
impl From<LineBreak> for DefinitionChild {
    fn from(child: LineBreak) -> Self {
        DefinitionChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for DefinitionChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        DefinitionChild::LineBreak(builder.build())
    }
}
impl From<Link> for DefinitionChild {
    fn from(child: Link) -> Self {
        DefinitionChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for DefinitionChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        DefinitionChild::Link(builder.build())
    }
}
impl From<Map> for DefinitionChild {
    fn from(child: Map) -> Self {
        DefinitionChild::Map(child)
    }
}
impl From<builders::MapBuilder> for DefinitionChild {
    fn from(builder: builders::MapBuilder) -> Self {
        DefinitionChild::Map(builder.build())
    }
}
impl From<MapArea> for DefinitionChild {
    fn from(child: MapArea) -> Self {
        DefinitionChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for DefinitionChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        DefinitionChild::MapArea(builder.build())
    }
}
impl From<Mark> for DefinitionChild {
    fn from(child: Mark) -> Self {
        DefinitionChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for DefinitionChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        DefinitionChild::Mark(builder.build())
    }
}
impl From<Metadata> for DefinitionChild {
    fn from(child: Metadata) -> Self {
        DefinitionChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for DefinitionChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        DefinitionChild::Metadata(builder.build())
    }
}
impl From<Meter> for DefinitionChild {
    fn from(child: Meter) -> Self {
        DefinitionChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for DefinitionChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        DefinitionChild::Meter(builder.build())
    }
}
impl From<NoScript> for DefinitionChild {
    fn from(child: NoScript) -> Self {
        DefinitionChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for DefinitionChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        DefinitionChild::NoScript(builder.build())
    }
}
impl From<Object> for DefinitionChild {
    fn from(child: Object) -> Self {
        DefinitionChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for DefinitionChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        DefinitionChild::Object(builder.build())
    }
}
impl From<Output> for DefinitionChild {
    fn from(child: Output) -> Self {
        DefinitionChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for DefinitionChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        DefinitionChild::Output(builder.build())
    }
}
impl From<Picture> for DefinitionChild {
    fn from(child: Picture) -> Self {
        DefinitionChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for DefinitionChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        DefinitionChild::Picture(builder.build())
    }
}
impl From<Progress> for DefinitionChild {
    fn from(child: Progress) -> Self {
        DefinitionChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for DefinitionChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        DefinitionChild::Progress(builder.build())
    }
}
impl From<Quote> for DefinitionChild {
    fn from(child: Quote) -> Self {
        DefinitionChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for DefinitionChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        DefinitionChild::Quote(builder.build())
    }
}
impl From<Ruby> for DefinitionChild {
    fn from(child: Ruby) -> Self {
        DefinitionChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for DefinitionChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        DefinitionChild::Ruby(builder.build())
    }
}
impl From<Sample> for DefinitionChild {
    fn from(child: Sample) -> Self {
        DefinitionChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for DefinitionChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        DefinitionChild::Sample(builder.build())
    }
}
impl From<Script> for DefinitionChild {
    fn from(child: Script) -> Self {
        DefinitionChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for DefinitionChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        DefinitionChild::Script(builder.build())
    }
}
impl From<Select> for DefinitionChild {
    fn from(child: Select) -> Self {
        DefinitionChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for DefinitionChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        DefinitionChild::Select(builder.build())
    }
}
impl From<Slot> for DefinitionChild {
    fn from(child: Slot) -> Self {
        DefinitionChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for DefinitionChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        DefinitionChild::Slot(builder.build())
    }
}
impl From<Small> for DefinitionChild {
    fn from(child: Small) -> Self {
        DefinitionChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for DefinitionChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        DefinitionChild::Small(builder.build())
    }
}
impl From<Span> for DefinitionChild {
    fn from(child: Span) -> Self {
        DefinitionChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for DefinitionChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        DefinitionChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for DefinitionChild {
    fn from(child: StrikeThrough) -> Self {
        DefinitionChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for DefinitionChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        DefinitionChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for DefinitionChild {
    fn from(child: Strong) -> Self {
        DefinitionChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for DefinitionChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        DefinitionChild::Strong(builder.build())
    }
}
impl From<SubScript> for DefinitionChild {
    fn from(child: SubScript) -> Self {
        DefinitionChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for DefinitionChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        DefinitionChild::SubScript(builder.build())
    }
}
impl From<SupScript> for DefinitionChild {
    fn from(child: SupScript) -> Self {
        DefinitionChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for DefinitionChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        DefinitionChild::SupScript(builder.build())
    }
}
impl From<Template> for DefinitionChild {
    fn from(child: Template) -> Self {
        DefinitionChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for DefinitionChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        DefinitionChild::Template(builder.build())
    }
}
impl From<TextArea> for DefinitionChild {
    fn from(child: TextArea) -> Self {
        DefinitionChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for DefinitionChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        DefinitionChild::TextArea(builder.build())
    }
}
impl From<Time> for DefinitionChild {
    fn from(child: Time) -> Self {
        DefinitionChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for DefinitionChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        DefinitionChild::Time(builder.build())
    }
}
impl From<Underline> for DefinitionChild {
    fn from(child: Underline) -> Self {
        DefinitionChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for DefinitionChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        DefinitionChild::Underline(builder.build())
    }
}
impl From<Variable> for DefinitionChild {
    fn from(child: Variable) -> Self {
        DefinitionChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for DefinitionChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        DefinitionChild::Variable(builder.build())
    }
}
impl From<Video> for DefinitionChild {
    fn from(child: Video) -> Self {
        DefinitionChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for DefinitionChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        DefinitionChild::Video(builder.build())
    }
}
impl From<WordBreak> for DefinitionChild {
    fn from(child: WordBreak) -> Self {
        DefinitionChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for DefinitionChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        DefinitionChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for DefinitionChild {
    fn from(s: &'static str) -> Self {
        DefinitionChild::Text(s.into())
    }
}
impl From<String> for DefinitionChild {
    fn from(s: String) -> Self {
        DefinitionChild::Text(s.into())
    }
}
impl From<CowStr> for DefinitionChild {
    fn from(s: CowStr) -> Self {
        DefinitionChild::Text(s)
    }
}
impl std::fmt::Debug for DefinitionChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DefinitionChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::BidirectionalIsolate(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            DefinitionChild::BidirectionalOverride(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            DefinitionChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Button(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Code(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Data(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Image(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Input(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Label(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Link(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Map(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Object(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Output(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Script(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Select(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Small(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Span(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Template(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Time(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Video(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            DefinitionChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for DefinitionChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DefinitionChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Audio(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DefinitionChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DefinitionChild::Bold(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Button(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Cite(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Code(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Custom(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Data(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::DataList(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Definition(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Embed(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Image(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Input(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Italic(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Label(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Link(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Map(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Mark(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Meter(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Object(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Output(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Picture(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Progress(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Quote(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Sample(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Script(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Select(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Slot(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Small(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Span(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Strong(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Template(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Time(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Underline(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Variable(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Video(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            DefinitionChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
