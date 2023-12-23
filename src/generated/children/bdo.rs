// 🤖 This file is generated!

use crate::*;
/// The `<bdo>` element's children.
#[derive(Clone)]
pub enum BidirectionalOverrideChild {
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
impl From<Abbreviation> for BidirectionalOverrideChild {
    fn from(child: Abbreviation) -> Self {
        BidirectionalOverrideChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        BidirectionalOverrideChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for BidirectionalOverrideChild {
    fn from(child: Anchor) -> Self {
        BidirectionalOverrideChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        BidirectionalOverrideChild::Anchor(builder.build())
    }
}
impl From<Audio> for BidirectionalOverrideChild {
    fn from(child: Audio) -> Self {
        BidirectionalOverrideChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        BidirectionalOverrideChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for BidirectionalOverrideChild {
    fn from(child: BidirectionalIsolate) -> Self {
        BidirectionalOverrideChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        BidirectionalOverrideChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for BidirectionalOverrideChild {
    fn from(child: BidirectionalOverride) -> Self {
        BidirectionalOverrideChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        BidirectionalOverrideChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for BidirectionalOverrideChild {
    fn from(child: Bold) -> Self {
        BidirectionalOverrideChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        BidirectionalOverrideChild::Bold(builder.build())
    }
}
impl From<Button> for BidirectionalOverrideChild {
    fn from(child: Button) -> Self {
        BidirectionalOverrideChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        BidirectionalOverrideChild::Button(builder.build())
    }
}
impl From<Canvas> for BidirectionalOverrideChild {
    fn from(child: Canvas) -> Self {
        BidirectionalOverrideChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        BidirectionalOverrideChild::Canvas(builder.build())
    }
}
impl From<Cite> for BidirectionalOverrideChild {
    fn from(child: Cite) -> Self {
        BidirectionalOverrideChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        BidirectionalOverrideChild::Cite(builder.build())
    }
}
impl From<Code> for BidirectionalOverrideChild {
    fn from(child: Code) -> Self {
        BidirectionalOverrideChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        BidirectionalOverrideChild::Code(builder.build())
    }
}
impl From<Custom> for BidirectionalOverrideChild {
    fn from(child: Custom) -> Self {
        BidirectionalOverrideChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        BidirectionalOverrideChild::Custom(builder.build())
    }
}
impl From<Data> for BidirectionalOverrideChild {
    fn from(child: Data) -> Self {
        BidirectionalOverrideChild::Data(child)
    }
}
impl From<builders::DataBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::DataBuilder) -> Self {
        BidirectionalOverrideChild::Data(builder.build())
    }
}
impl From<DataList> for BidirectionalOverrideChild {
    fn from(child: DataList) -> Self {
        BidirectionalOverrideChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        BidirectionalOverrideChild::DataList(builder.build())
    }
}
impl From<Definition> for BidirectionalOverrideChild {
    fn from(child: Definition) -> Self {
        BidirectionalOverrideChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        BidirectionalOverrideChild::Definition(builder.build())
    }
}
impl From<Deleted> for BidirectionalOverrideChild {
    fn from(child: Deleted) -> Self {
        BidirectionalOverrideChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        BidirectionalOverrideChild::Deleted(builder.build())
    }
}
impl From<Embed> for BidirectionalOverrideChild {
    fn from(child: Embed) -> Self {
        BidirectionalOverrideChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        BidirectionalOverrideChild::Embed(builder.build())
    }
}
impl From<Emphasis> for BidirectionalOverrideChild {
    fn from(child: Emphasis) -> Self {
        BidirectionalOverrideChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        BidirectionalOverrideChild::Emphasis(builder.build())
    }
}
impl From<Image> for BidirectionalOverrideChild {
    fn from(child: Image) -> Self {
        BidirectionalOverrideChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        BidirectionalOverrideChild::Image(builder.build())
    }
}
impl From<InlineFrame> for BidirectionalOverrideChild {
    fn from(child: InlineFrame) -> Self {
        BidirectionalOverrideChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        BidirectionalOverrideChild::InlineFrame(builder.build())
    }
}
impl From<Input> for BidirectionalOverrideChild {
    fn from(child: Input) -> Self {
        BidirectionalOverrideChild::Input(child)
    }
}
impl From<builders::InputBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::InputBuilder) -> Self {
        BidirectionalOverrideChild::Input(builder.build())
    }
}
impl From<Inserted> for BidirectionalOverrideChild {
    fn from(child: Inserted) -> Self {
        BidirectionalOverrideChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        BidirectionalOverrideChild::Inserted(builder.build())
    }
}
impl From<Italic> for BidirectionalOverrideChild {
    fn from(child: Italic) -> Self {
        BidirectionalOverrideChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        BidirectionalOverrideChild::Italic(builder.build())
    }
}
impl From<Keyboard> for BidirectionalOverrideChild {
    fn from(child: Keyboard) -> Self {
        BidirectionalOverrideChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        BidirectionalOverrideChild::Keyboard(builder.build())
    }
}
impl From<Label> for BidirectionalOverrideChild {
    fn from(child: Label) -> Self {
        BidirectionalOverrideChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        BidirectionalOverrideChild::Label(builder.build())
    }
}
impl From<LineBreak> for BidirectionalOverrideChild {
    fn from(child: LineBreak) -> Self {
        BidirectionalOverrideChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        BidirectionalOverrideChild::LineBreak(builder.build())
    }
}
impl From<Link> for BidirectionalOverrideChild {
    fn from(child: Link) -> Self {
        BidirectionalOverrideChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        BidirectionalOverrideChild::Link(builder.build())
    }
}
impl From<Map> for BidirectionalOverrideChild {
    fn from(child: Map) -> Self {
        BidirectionalOverrideChild::Map(child)
    }
}
impl From<builders::MapBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::MapBuilder) -> Self {
        BidirectionalOverrideChild::Map(builder.build())
    }
}
impl From<MapArea> for BidirectionalOverrideChild {
    fn from(child: MapArea) -> Self {
        BidirectionalOverrideChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        BidirectionalOverrideChild::MapArea(builder.build())
    }
}
impl From<Mark> for BidirectionalOverrideChild {
    fn from(child: Mark) -> Self {
        BidirectionalOverrideChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        BidirectionalOverrideChild::Mark(builder.build())
    }
}
impl From<Metadata> for BidirectionalOverrideChild {
    fn from(child: Metadata) -> Self {
        BidirectionalOverrideChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        BidirectionalOverrideChild::Metadata(builder.build())
    }
}
impl From<Meter> for BidirectionalOverrideChild {
    fn from(child: Meter) -> Self {
        BidirectionalOverrideChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        BidirectionalOverrideChild::Meter(builder.build())
    }
}
impl From<NoScript> for BidirectionalOverrideChild {
    fn from(child: NoScript) -> Self {
        BidirectionalOverrideChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        BidirectionalOverrideChild::NoScript(builder.build())
    }
}
impl From<Object> for BidirectionalOverrideChild {
    fn from(child: Object) -> Self {
        BidirectionalOverrideChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        BidirectionalOverrideChild::Object(builder.build())
    }
}
impl From<Output> for BidirectionalOverrideChild {
    fn from(child: Output) -> Self {
        BidirectionalOverrideChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        BidirectionalOverrideChild::Output(builder.build())
    }
}
impl From<Picture> for BidirectionalOverrideChild {
    fn from(child: Picture) -> Self {
        BidirectionalOverrideChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        BidirectionalOverrideChild::Picture(builder.build())
    }
}
impl From<Progress> for BidirectionalOverrideChild {
    fn from(child: Progress) -> Self {
        BidirectionalOverrideChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        BidirectionalOverrideChild::Progress(builder.build())
    }
}
impl From<Quote> for BidirectionalOverrideChild {
    fn from(child: Quote) -> Self {
        BidirectionalOverrideChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        BidirectionalOverrideChild::Quote(builder.build())
    }
}
impl From<Ruby> for BidirectionalOverrideChild {
    fn from(child: Ruby) -> Self {
        BidirectionalOverrideChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        BidirectionalOverrideChild::Ruby(builder.build())
    }
}
impl From<Sample> for BidirectionalOverrideChild {
    fn from(child: Sample) -> Self {
        BidirectionalOverrideChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        BidirectionalOverrideChild::Sample(builder.build())
    }
}
impl From<Script> for BidirectionalOverrideChild {
    fn from(child: Script) -> Self {
        BidirectionalOverrideChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        BidirectionalOverrideChild::Script(builder.build())
    }
}
impl From<Select> for BidirectionalOverrideChild {
    fn from(child: Select) -> Self {
        BidirectionalOverrideChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        BidirectionalOverrideChild::Select(builder.build())
    }
}
impl From<Slot> for BidirectionalOverrideChild {
    fn from(child: Slot) -> Self {
        BidirectionalOverrideChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        BidirectionalOverrideChild::Slot(builder.build())
    }
}
impl From<Small> for BidirectionalOverrideChild {
    fn from(child: Small) -> Self {
        BidirectionalOverrideChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        BidirectionalOverrideChild::Small(builder.build())
    }
}
impl From<Span> for BidirectionalOverrideChild {
    fn from(child: Span) -> Self {
        BidirectionalOverrideChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        BidirectionalOverrideChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for BidirectionalOverrideChild {
    fn from(child: StrikeThrough) -> Self {
        BidirectionalOverrideChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        BidirectionalOverrideChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for BidirectionalOverrideChild {
    fn from(child: Strong) -> Self {
        BidirectionalOverrideChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        BidirectionalOverrideChild::Strong(builder.build())
    }
}
impl From<SubScript> for BidirectionalOverrideChild {
    fn from(child: SubScript) -> Self {
        BidirectionalOverrideChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        BidirectionalOverrideChild::SubScript(builder.build())
    }
}
impl From<SupScript> for BidirectionalOverrideChild {
    fn from(child: SupScript) -> Self {
        BidirectionalOverrideChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        BidirectionalOverrideChild::SupScript(builder.build())
    }
}
impl From<Template> for BidirectionalOverrideChild {
    fn from(child: Template) -> Self {
        BidirectionalOverrideChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        BidirectionalOverrideChild::Template(builder.build())
    }
}
impl From<TextArea> for BidirectionalOverrideChild {
    fn from(child: TextArea) -> Self {
        BidirectionalOverrideChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        BidirectionalOverrideChild::TextArea(builder.build())
    }
}
impl From<Time> for BidirectionalOverrideChild {
    fn from(child: Time) -> Self {
        BidirectionalOverrideChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        BidirectionalOverrideChild::Time(builder.build())
    }
}
impl From<Underline> for BidirectionalOverrideChild {
    fn from(child: Underline) -> Self {
        BidirectionalOverrideChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        BidirectionalOverrideChild::Underline(builder.build())
    }
}
impl From<Variable> for BidirectionalOverrideChild {
    fn from(child: Variable) -> Self {
        BidirectionalOverrideChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        BidirectionalOverrideChild::Variable(builder.build())
    }
}
impl From<Video> for BidirectionalOverrideChild {
    fn from(child: Video) -> Self {
        BidirectionalOverrideChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        BidirectionalOverrideChild::Video(builder.build())
    }
}
impl From<WordBreak> for BidirectionalOverrideChild {
    fn from(child: WordBreak) -> Self {
        BidirectionalOverrideChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for BidirectionalOverrideChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        BidirectionalOverrideChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for BidirectionalOverrideChild {
    fn from(s: &'static str) -> Self {
        BidirectionalOverrideChild::Text(s.into())
    }
}
impl From<String> for BidirectionalOverrideChild {
    fn from(s: String) -> Self {
        BidirectionalOverrideChild::Text(s.into())
    }
}
impl From<CowStr> for BidirectionalOverrideChild {
    fn from(s: CowStr) -> Self {
        BidirectionalOverrideChild::Text(s)
    }
}
impl std::fmt::Debug for BidirectionalOverrideChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BidirectionalOverrideChild::Abbreviation(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            BidirectionalOverrideChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::BidirectionalIsolate(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            BidirectionalOverrideChild::BidirectionalOverride(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            BidirectionalOverrideChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Button(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Code(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Data(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Definition(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            BidirectionalOverrideChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Image(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::InlineFrame(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            BidirectionalOverrideChild::Input(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Label(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::LineBreak(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            BidirectionalOverrideChild::Link(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Map(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Object(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Output(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Script(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Select(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Small(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Span(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::StrikeThrough(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            BidirectionalOverrideChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::SubScript(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            BidirectionalOverrideChild::SupScript(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            BidirectionalOverrideChild::Template(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Time(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Underline(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            BidirectionalOverrideChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::Video(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalOverrideChild::WordBreak(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            BidirectionalOverrideChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for BidirectionalOverrideChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BidirectionalOverrideChild::Abbreviation(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Audio(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::Bold(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Button(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Cite(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Code(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Custom(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Data(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::DataList(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::Definition(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::Deleted(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::Embed(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Emphasis(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::Image(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::InlineFrame(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::Input(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Inserted(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::Italic(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Keyboard(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::Label(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::LineBreak(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::Link(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Map(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::MapArea(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::Mark(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Metadata(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::Meter(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::NoScript(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::Object(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Output(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Picture(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::Progress(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::Quote(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Sample(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Script(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Select(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Slot(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Small(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Span(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::StrikeThrough(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::Strong(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::SubScript(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::SupScript(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::Template(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::TextArea(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::Time(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::Underline(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::Variable(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::Video(child) => std::fmt::Display::fmt(child, f),
            BidirectionalOverrideChild::WordBreak(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalOverrideChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
