// 🤖 This file is generated!

use crate::*;
/// The `<main>` element's children.
#[derive(Clone)]
pub enum MainChild {
    Abbreviation(Abbreviation),
    Address(Address),
    Anchor(Anchor),
    Article(Article),
    Aside(Aside),
    Audio(Audio),
    BidirectionalIsolate(BidirectionalIsolate),
    BidirectionalOverride(BidirectionalOverride),
    BlockQuote(BlockQuote),
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
    DescriptionList(DescriptionList),
    Details(Details),
    Dialog(Dialog),
    Division(Division),
    Embed(Embed),
    Emphasis(Emphasis),
    FieldSet(FieldSet),
    Figure(Figure),
    Footer(Footer),
    Form(Form),
    Header(Header),
    Heading1(Heading1),
    Heading2(Heading2),
    Heading3(Heading3),
    Heading4(Heading4),
    Heading5(Heading5),
    Heading6(Heading6),
    HeadingGroup(HeadingGroup),
    HorizontalRule(HorizontalRule),
    Image(Image),
    InlineFrame(InlineFrame),
    Input(Input),
    Inserted(Inserted),
    Italic(Italic),
    Keyboard(Keyboard),
    Label(Label),
    LineBreak(LineBreak),
    Link(Link),
    Main(Main),
    Map(Map),
    MapArea(MapArea),
    Mark(Mark),
    Menu(Menu),
    Metadata(Metadata),
    Meter(Meter),
    Navigation(Navigation),
    NoScript(NoScript),
    Object(Object),
    OrderedList(OrderedList),
    Output(Output),
    Paragraph(Paragraph),
    Picture(Picture),
    Preformatted(Preformatted),
    Progress(Progress),
    Quote(Quote),
    Ruby(Ruby),
    Sample(Sample),
    Script(Script),
    Search(Search),
    Section(Section),
    Select(Select),
    Slot(Slot),
    Small(Small),
    Span(Span),
    StrikeThrough(StrikeThrough),
    Strong(Strong),
    SubScript(SubScript),
    SupScript(SupScript),
    Table(Table),
    Template(Template),
    TextArea(TextArea),
    Time(Time),
    Underline(Underline),
    UnorderedList(UnorderedList),
    Variable(Variable),
    Video(Video),
    WordBreak(WordBreak),
    Text(CowStr),
}
impl From<Abbreviation> for MainChild {
    fn from(child: Abbreviation) -> Self {
        MainChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for MainChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        MainChild::Abbreviation(builder.build())
    }
}
impl From<Address> for MainChild {
    fn from(child: Address) -> Self {
        MainChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for MainChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        MainChild::Address(builder.build())
    }
}
impl From<Anchor> for MainChild {
    fn from(child: Anchor) -> Self {
        MainChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for MainChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        MainChild::Anchor(builder.build())
    }
}
impl From<Article> for MainChild {
    fn from(child: Article) -> Self {
        MainChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for MainChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        MainChild::Article(builder.build())
    }
}
impl From<Aside> for MainChild {
    fn from(child: Aside) -> Self {
        MainChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for MainChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        MainChild::Aside(builder.build())
    }
}
impl From<Audio> for MainChild {
    fn from(child: Audio) -> Self {
        MainChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for MainChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        MainChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for MainChild {
    fn from(child: BidirectionalIsolate) -> Self {
        MainChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for MainChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        MainChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for MainChild {
    fn from(child: BidirectionalOverride) -> Self {
        MainChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for MainChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        MainChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for MainChild {
    fn from(child: BlockQuote) -> Self {
        MainChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for MainChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        MainChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for MainChild {
    fn from(child: Bold) -> Self {
        MainChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for MainChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        MainChild::Bold(builder.build())
    }
}
impl From<Button> for MainChild {
    fn from(child: Button) -> Self {
        MainChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for MainChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        MainChild::Button(builder.build())
    }
}
impl From<Canvas> for MainChild {
    fn from(child: Canvas) -> Self {
        MainChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for MainChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        MainChild::Canvas(builder.build())
    }
}
impl From<Cite> for MainChild {
    fn from(child: Cite) -> Self {
        MainChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for MainChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        MainChild::Cite(builder.build())
    }
}
impl From<Code> for MainChild {
    fn from(child: Code) -> Self {
        MainChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for MainChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        MainChild::Code(builder.build())
    }
}
impl From<Custom> for MainChild {
    fn from(child: Custom) -> Self {
        MainChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for MainChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        MainChild::Custom(builder.build())
    }
}
impl From<Data> for MainChild {
    fn from(child: Data) -> Self {
        MainChild::Data(child)
    }
}
impl From<builders::DataBuilder> for MainChild {
    fn from(builder: builders::DataBuilder) -> Self {
        MainChild::Data(builder.build())
    }
}
impl From<DataList> for MainChild {
    fn from(child: DataList) -> Self {
        MainChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for MainChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        MainChild::DataList(builder.build())
    }
}
impl From<Definition> for MainChild {
    fn from(child: Definition) -> Self {
        MainChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for MainChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        MainChild::Definition(builder.build())
    }
}
impl From<Deleted> for MainChild {
    fn from(child: Deleted) -> Self {
        MainChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for MainChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        MainChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for MainChild {
    fn from(child: DescriptionList) -> Self {
        MainChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for MainChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        MainChild::DescriptionList(builder.build())
    }
}
impl From<Details> for MainChild {
    fn from(child: Details) -> Self {
        MainChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for MainChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        MainChild::Details(builder.build())
    }
}
impl From<Dialog> for MainChild {
    fn from(child: Dialog) -> Self {
        MainChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for MainChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        MainChild::Dialog(builder.build())
    }
}
impl From<Division> for MainChild {
    fn from(child: Division) -> Self {
        MainChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for MainChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        MainChild::Division(builder.build())
    }
}
impl From<Embed> for MainChild {
    fn from(child: Embed) -> Self {
        MainChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for MainChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        MainChild::Embed(builder.build())
    }
}
impl From<Emphasis> for MainChild {
    fn from(child: Emphasis) -> Self {
        MainChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for MainChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        MainChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for MainChild {
    fn from(child: FieldSet) -> Self {
        MainChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for MainChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        MainChild::FieldSet(builder.build())
    }
}
impl From<Figure> for MainChild {
    fn from(child: Figure) -> Self {
        MainChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for MainChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        MainChild::Figure(builder.build())
    }
}
impl From<Footer> for MainChild {
    fn from(child: Footer) -> Self {
        MainChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for MainChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        MainChild::Footer(builder.build())
    }
}
impl From<Form> for MainChild {
    fn from(child: Form) -> Self {
        MainChild::Form(child)
    }
}
impl From<builders::FormBuilder> for MainChild {
    fn from(builder: builders::FormBuilder) -> Self {
        MainChild::Form(builder.build())
    }
}
impl From<Header> for MainChild {
    fn from(child: Header) -> Self {
        MainChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for MainChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        MainChild::Header(builder.build())
    }
}
impl From<Heading1> for MainChild {
    fn from(child: Heading1) -> Self {
        MainChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for MainChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        MainChild::Heading1(builder.build())
    }
}
impl From<Heading2> for MainChild {
    fn from(child: Heading2) -> Self {
        MainChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for MainChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        MainChild::Heading2(builder.build())
    }
}
impl From<Heading3> for MainChild {
    fn from(child: Heading3) -> Self {
        MainChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for MainChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        MainChild::Heading3(builder.build())
    }
}
impl From<Heading4> for MainChild {
    fn from(child: Heading4) -> Self {
        MainChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for MainChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        MainChild::Heading4(builder.build())
    }
}
impl From<Heading5> for MainChild {
    fn from(child: Heading5) -> Self {
        MainChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for MainChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        MainChild::Heading5(builder.build())
    }
}
impl From<Heading6> for MainChild {
    fn from(child: Heading6) -> Self {
        MainChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for MainChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        MainChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for MainChild {
    fn from(child: HeadingGroup) -> Self {
        MainChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for MainChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        MainChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for MainChild {
    fn from(child: HorizontalRule) -> Self {
        MainChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for MainChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        MainChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for MainChild {
    fn from(child: Image) -> Self {
        MainChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for MainChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        MainChild::Image(builder.build())
    }
}
impl From<InlineFrame> for MainChild {
    fn from(child: InlineFrame) -> Self {
        MainChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for MainChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        MainChild::InlineFrame(builder.build())
    }
}
impl From<Input> for MainChild {
    fn from(child: Input) -> Self {
        MainChild::Input(child)
    }
}
impl From<builders::InputBuilder> for MainChild {
    fn from(builder: builders::InputBuilder) -> Self {
        MainChild::Input(builder.build())
    }
}
impl From<Inserted> for MainChild {
    fn from(child: Inserted) -> Self {
        MainChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for MainChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        MainChild::Inserted(builder.build())
    }
}
impl From<Italic> for MainChild {
    fn from(child: Italic) -> Self {
        MainChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for MainChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        MainChild::Italic(builder.build())
    }
}
impl From<Keyboard> for MainChild {
    fn from(child: Keyboard) -> Self {
        MainChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for MainChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        MainChild::Keyboard(builder.build())
    }
}
impl From<Label> for MainChild {
    fn from(child: Label) -> Self {
        MainChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for MainChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        MainChild::Label(builder.build())
    }
}
impl From<LineBreak> for MainChild {
    fn from(child: LineBreak) -> Self {
        MainChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for MainChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        MainChild::LineBreak(builder.build())
    }
}
impl From<Link> for MainChild {
    fn from(child: Link) -> Self {
        MainChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for MainChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        MainChild::Link(builder.build())
    }
}
impl From<Main> for MainChild {
    fn from(child: Main) -> Self {
        MainChild::Main(child)
    }
}
impl From<builders::MainBuilder> for MainChild {
    fn from(builder: builders::MainBuilder) -> Self {
        MainChild::Main(builder.build())
    }
}
impl From<Map> for MainChild {
    fn from(child: Map) -> Self {
        MainChild::Map(child)
    }
}
impl From<builders::MapBuilder> for MainChild {
    fn from(builder: builders::MapBuilder) -> Self {
        MainChild::Map(builder.build())
    }
}
impl From<MapArea> for MainChild {
    fn from(child: MapArea) -> Self {
        MainChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for MainChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        MainChild::MapArea(builder.build())
    }
}
impl From<Mark> for MainChild {
    fn from(child: Mark) -> Self {
        MainChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for MainChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        MainChild::Mark(builder.build())
    }
}
impl From<Menu> for MainChild {
    fn from(child: Menu) -> Self {
        MainChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for MainChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        MainChild::Menu(builder.build())
    }
}
impl From<Metadata> for MainChild {
    fn from(child: Metadata) -> Self {
        MainChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for MainChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        MainChild::Metadata(builder.build())
    }
}
impl From<Meter> for MainChild {
    fn from(child: Meter) -> Self {
        MainChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for MainChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        MainChild::Meter(builder.build())
    }
}
impl From<Navigation> for MainChild {
    fn from(child: Navigation) -> Self {
        MainChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for MainChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        MainChild::Navigation(builder.build())
    }
}
impl From<NoScript> for MainChild {
    fn from(child: NoScript) -> Self {
        MainChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for MainChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        MainChild::NoScript(builder.build())
    }
}
impl From<Object> for MainChild {
    fn from(child: Object) -> Self {
        MainChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for MainChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        MainChild::Object(builder.build())
    }
}
impl From<OrderedList> for MainChild {
    fn from(child: OrderedList) -> Self {
        MainChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for MainChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        MainChild::OrderedList(builder.build())
    }
}
impl From<Output> for MainChild {
    fn from(child: Output) -> Self {
        MainChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for MainChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        MainChild::Output(builder.build())
    }
}
impl From<Paragraph> for MainChild {
    fn from(child: Paragraph) -> Self {
        MainChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for MainChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        MainChild::Paragraph(builder.build())
    }
}
impl From<Picture> for MainChild {
    fn from(child: Picture) -> Self {
        MainChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for MainChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        MainChild::Picture(builder.build())
    }
}
impl From<Preformatted> for MainChild {
    fn from(child: Preformatted) -> Self {
        MainChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for MainChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        MainChild::Preformatted(builder.build())
    }
}
impl From<Progress> for MainChild {
    fn from(child: Progress) -> Self {
        MainChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for MainChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        MainChild::Progress(builder.build())
    }
}
impl From<Quote> for MainChild {
    fn from(child: Quote) -> Self {
        MainChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for MainChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        MainChild::Quote(builder.build())
    }
}
impl From<Ruby> for MainChild {
    fn from(child: Ruby) -> Self {
        MainChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for MainChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        MainChild::Ruby(builder.build())
    }
}
impl From<Sample> for MainChild {
    fn from(child: Sample) -> Self {
        MainChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for MainChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        MainChild::Sample(builder.build())
    }
}
impl From<Script> for MainChild {
    fn from(child: Script) -> Self {
        MainChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for MainChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        MainChild::Script(builder.build())
    }
}
impl From<Search> for MainChild {
    fn from(child: Search) -> Self {
        MainChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for MainChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        MainChild::Search(builder.build())
    }
}
impl From<Section> for MainChild {
    fn from(child: Section) -> Self {
        MainChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for MainChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        MainChild::Section(builder.build())
    }
}
impl From<Select> for MainChild {
    fn from(child: Select) -> Self {
        MainChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for MainChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        MainChild::Select(builder.build())
    }
}
impl From<Slot> for MainChild {
    fn from(child: Slot) -> Self {
        MainChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for MainChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        MainChild::Slot(builder.build())
    }
}
impl From<Small> for MainChild {
    fn from(child: Small) -> Self {
        MainChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for MainChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        MainChild::Small(builder.build())
    }
}
impl From<Span> for MainChild {
    fn from(child: Span) -> Self {
        MainChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for MainChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        MainChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for MainChild {
    fn from(child: StrikeThrough) -> Self {
        MainChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for MainChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        MainChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for MainChild {
    fn from(child: Strong) -> Self {
        MainChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for MainChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        MainChild::Strong(builder.build())
    }
}
impl From<SubScript> for MainChild {
    fn from(child: SubScript) -> Self {
        MainChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for MainChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        MainChild::SubScript(builder.build())
    }
}
impl From<SupScript> for MainChild {
    fn from(child: SupScript) -> Self {
        MainChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for MainChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        MainChild::SupScript(builder.build())
    }
}
impl From<Table> for MainChild {
    fn from(child: Table) -> Self {
        MainChild::Table(child)
    }
}
impl From<builders::TableBuilder> for MainChild {
    fn from(builder: builders::TableBuilder) -> Self {
        MainChild::Table(builder.build())
    }
}
impl From<Template> for MainChild {
    fn from(child: Template) -> Self {
        MainChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for MainChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        MainChild::Template(builder.build())
    }
}
impl From<TextArea> for MainChild {
    fn from(child: TextArea) -> Self {
        MainChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for MainChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        MainChild::TextArea(builder.build())
    }
}
impl From<Time> for MainChild {
    fn from(child: Time) -> Self {
        MainChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for MainChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        MainChild::Time(builder.build())
    }
}
impl From<Underline> for MainChild {
    fn from(child: Underline) -> Self {
        MainChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for MainChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        MainChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for MainChild {
    fn from(child: UnorderedList) -> Self {
        MainChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for MainChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        MainChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for MainChild {
    fn from(child: Variable) -> Self {
        MainChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for MainChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        MainChild::Variable(builder.build())
    }
}
impl From<Video> for MainChild {
    fn from(child: Video) -> Self {
        MainChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for MainChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        MainChild::Video(builder.build())
    }
}
impl From<WordBreak> for MainChild {
    fn from(child: WordBreak) -> Self {
        MainChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for MainChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        MainChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for MainChild {
    fn from(s: &'static str) -> Self {
        MainChild::Text(s.into())
    }
}
impl From<String> for MainChild {
    fn from(s: String) -> Self {
        MainChild::Text(s.into())
    }
}
impl From<CowStr> for MainChild {
    fn from(s: CowStr) -> Self {
        MainChild::Text(s)
    }
}
impl std::fmt::Debug for MainChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Address(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Article(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            MainChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            MainChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            MainChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Button(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Code(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Data(child) => std::fmt::Debug::fmt(child, f),
            MainChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            MainChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Details(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Division(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            MainChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Form(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Header(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            MainChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            MainChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Image(child) => std::fmt::Debug::fmt(child, f),
            MainChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Input(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Label(child) => std::fmt::Debug::fmt(child, f),
            MainChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Link(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Main(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Map(child) => std::fmt::Debug::fmt(child, f),
            MainChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            MainChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Object(child) => std::fmt::Debug::fmt(child, f),
            MainChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Output(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Script(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Search(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Section(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Select(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Small(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Span(child) => std::fmt::Debug::fmt(child, f),
            MainChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            MainChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            MainChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Table(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Template(child) => std::fmt::Debug::fmt(child, f),
            MainChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Time(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            MainChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Video(child) => std::fmt::Debug::fmt(child, f),
            MainChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            MainChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for MainChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            MainChild::Address(child) => std::fmt::Display::fmt(child, f),
            MainChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            MainChild::Article(child) => std::fmt::Display::fmt(child, f),
            MainChild::Aside(child) => std::fmt::Display::fmt(child, f),
            MainChild::Audio(child) => std::fmt::Display::fmt(child, f),
            MainChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            MainChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            MainChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            MainChild::Bold(child) => std::fmt::Display::fmt(child, f),
            MainChild::Button(child) => std::fmt::Display::fmt(child, f),
            MainChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            MainChild::Cite(child) => std::fmt::Display::fmt(child, f),
            MainChild::Code(child) => std::fmt::Display::fmt(child, f),
            MainChild::Custom(child) => std::fmt::Display::fmt(child, f),
            MainChild::Data(child) => std::fmt::Display::fmt(child, f),
            MainChild::DataList(child) => std::fmt::Display::fmt(child, f),
            MainChild::Definition(child) => std::fmt::Display::fmt(child, f),
            MainChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            MainChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            MainChild::Details(child) => std::fmt::Display::fmt(child, f),
            MainChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            MainChild::Division(child) => std::fmt::Display::fmt(child, f),
            MainChild::Embed(child) => std::fmt::Display::fmt(child, f),
            MainChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            MainChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            MainChild::Figure(child) => std::fmt::Display::fmt(child, f),
            MainChild::Footer(child) => std::fmt::Display::fmt(child, f),
            MainChild::Form(child) => std::fmt::Display::fmt(child, f),
            MainChild::Header(child) => std::fmt::Display::fmt(child, f),
            MainChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            MainChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            MainChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            MainChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            MainChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            MainChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            MainChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            MainChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            MainChild::Image(child) => std::fmt::Display::fmt(child, f),
            MainChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            MainChild::Input(child) => std::fmt::Display::fmt(child, f),
            MainChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            MainChild::Italic(child) => std::fmt::Display::fmt(child, f),
            MainChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            MainChild::Label(child) => std::fmt::Display::fmt(child, f),
            MainChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            MainChild::Link(child) => std::fmt::Display::fmt(child, f),
            MainChild::Main(child) => std::fmt::Display::fmt(child, f),
            MainChild::Map(child) => std::fmt::Display::fmt(child, f),
            MainChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            MainChild::Mark(child) => std::fmt::Display::fmt(child, f),
            MainChild::Menu(child) => std::fmt::Display::fmt(child, f),
            MainChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            MainChild::Meter(child) => std::fmt::Display::fmt(child, f),
            MainChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            MainChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            MainChild::Object(child) => std::fmt::Display::fmt(child, f),
            MainChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            MainChild::Output(child) => std::fmt::Display::fmt(child, f),
            MainChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            MainChild::Picture(child) => std::fmt::Display::fmt(child, f),
            MainChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            MainChild::Progress(child) => std::fmt::Display::fmt(child, f),
            MainChild::Quote(child) => std::fmt::Display::fmt(child, f),
            MainChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            MainChild::Sample(child) => std::fmt::Display::fmt(child, f),
            MainChild::Script(child) => std::fmt::Display::fmt(child, f),
            MainChild::Search(child) => std::fmt::Display::fmt(child, f),
            MainChild::Section(child) => std::fmt::Display::fmt(child, f),
            MainChild::Select(child) => std::fmt::Display::fmt(child, f),
            MainChild::Slot(child) => std::fmt::Display::fmt(child, f),
            MainChild::Small(child) => std::fmt::Display::fmt(child, f),
            MainChild::Span(child) => std::fmt::Display::fmt(child, f),
            MainChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            MainChild::Strong(child) => std::fmt::Display::fmt(child, f),
            MainChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            MainChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            MainChild::Table(child) => std::fmt::Display::fmt(child, f),
            MainChild::Template(child) => std::fmt::Display::fmt(child, f),
            MainChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            MainChild::Time(child) => std::fmt::Display::fmt(child, f),
            MainChild::Underline(child) => std::fmt::Display::fmt(child, f),
            MainChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            MainChild::Variable(child) => std::fmt::Display::fmt(child, f),
            MainChild::Video(child) => std::fmt::Display::fmt(child, f),
            MainChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            MainChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
