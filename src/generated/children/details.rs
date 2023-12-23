// 🤖 This file is generated!

use crate::*;
/// The `<details>` element's children.
#[derive(Clone)]
pub enum DetailsChild {
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
    Summary(Summary),
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
impl From<Abbreviation> for DetailsChild {
    fn from(child: Abbreviation) -> Self {
        DetailsChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for DetailsChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        DetailsChild::Abbreviation(builder.build())
    }
}
impl From<Address> for DetailsChild {
    fn from(child: Address) -> Self {
        DetailsChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for DetailsChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        DetailsChild::Address(builder.build())
    }
}
impl From<Anchor> for DetailsChild {
    fn from(child: Anchor) -> Self {
        DetailsChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for DetailsChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        DetailsChild::Anchor(builder.build())
    }
}
impl From<Article> for DetailsChild {
    fn from(child: Article) -> Self {
        DetailsChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for DetailsChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        DetailsChild::Article(builder.build())
    }
}
impl From<Aside> for DetailsChild {
    fn from(child: Aside) -> Self {
        DetailsChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for DetailsChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        DetailsChild::Aside(builder.build())
    }
}
impl From<Audio> for DetailsChild {
    fn from(child: Audio) -> Self {
        DetailsChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for DetailsChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        DetailsChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for DetailsChild {
    fn from(child: BidirectionalIsolate) -> Self {
        DetailsChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for DetailsChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        DetailsChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for DetailsChild {
    fn from(child: BidirectionalOverride) -> Self {
        DetailsChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for DetailsChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        DetailsChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for DetailsChild {
    fn from(child: BlockQuote) -> Self {
        DetailsChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for DetailsChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        DetailsChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for DetailsChild {
    fn from(child: Bold) -> Self {
        DetailsChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for DetailsChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        DetailsChild::Bold(builder.build())
    }
}
impl From<Button> for DetailsChild {
    fn from(child: Button) -> Self {
        DetailsChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for DetailsChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        DetailsChild::Button(builder.build())
    }
}
impl From<Canvas> for DetailsChild {
    fn from(child: Canvas) -> Self {
        DetailsChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for DetailsChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        DetailsChild::Canvas(builder.build())
    }
}
impl From<Cite> for DetailsChild {
    fn from(child: Cite) -> Self {
        DetailsChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for DetailsChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        DetailsChild::Cite(builder.build())
    }
}
impl From<Code> for DetailsChild {
    fn from(child: Code) -> Self {
        DetailsChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for DetailsChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        DetailsChild::Code(builder.build())
    }
}
impl From<Custom> for DetailsChild {
    fn from(child: Custom) -> Self {
        DetailsChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for DetailsChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        DetailsChild::Custom(builder.build())
    }
}
impl From<Data> for DetailsChild {
    fn from(child: Data) -> Self {
        DetailsChild::Data(child)
    }
}
impl From<builders::DataBuilder> for DetailsChild {
    fn from(builder: builders::DataBuilder) -> Self {
        DetailsChild::Data(builder.build())
    }
}
impl From<DataList> for DetailsChild {
    fn from(child: DataList) -> Self {
        DetailsChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for DetailsChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        DetailsChild::DataList(builder.build())
    }
}
impl From<Definition> for DetailsChild {
    fn from(child: Definition) -> Self {
        DetailsChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for DetailsChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        DetailsChild::Definition(builder.build())
    }
}
impl From<Deleted> for DetailsChild {
    fn from(child: Deleted) -> Self {
        DetailsChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for DetailsChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        DetailsChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for DetailsChild {
    fn from(child: DescriptionList) -> Self {
        DetailsChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for DetailsChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        DetailsChild::DescriptionList(builder.build())
    }
}
impl From<Details> for DetailsChild {
    fn from(child: Details) -> Self {
        DetailsChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for DetailsChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        DetailsChild::Details(builder.build())
    }
}
impl From<Dialog> for DetailsChild {
    fn from(child: Dialog) -> Self {
        DetailsChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for DetailsChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        DetailsChild::Dialog(builder.build())
    }
}
impl From<Division> for DetailsChild {
    fn from(child: Division) -> Self {
        DetailsChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for DetailsChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        DetailsChild::Division(builder.build())
    }
}
impl From<Embed> for DetailsChild {
    fn from(child: Embed) -> Self {
        DetailsChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for DetailsChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        DetailsChild::Embed(builder.build())
    }
}
impl From<Emphasis> for DetailsChild {
    fn from(child: Emphasis) -> Self {
        DetailsChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for DetailsChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        DetailsChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for DetailsChild {
    fn from(child: FieldSet) -> Self {
        DetailsChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for DetailsChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        DetailsChild::FieldSet(builder.build())
    }
}
impl From<Figure> for DetailsChild {
    fn from(child: Figure) -> Self {
        DetailsChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for DetailsChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        DetailsChild::Figure(builder.build())
    }
}
impl From<Footer> for DetailsChild {
    fn from(child: Footer) -> Self {
        DetailsChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for DetailsChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        DetailsChild::Footer(builder.build())
    }
}
impl From<Form> for DetailsChild {
    fn from(child: Form) -> Self {
        DetailsChild::Form(child)
    }
}
impl From<builders::FormBuilder> for DetailsChild {
    fn from(builder: builders::FormBuilder) -> Self {
        DetailsChild::Form(builder.build())
    }
}
impl From<Header> for DetailsChild {
    fn from(child: Header) -> Self {
        DetailsChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for DetailsChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        DetailsChild::Header(builder.build())
    }
}
impl From<Heading1> for DetailsChild {
    fn from(child: Heading1) -> Self {
        DetailsChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for DetailsChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        DetailsChild::Heading1(builder.build())
    }
}
impl From<Heading2> for DetailsChild {
    fn from(child: Heading2) -> Self {
        DetailsChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for DetailsChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        DetailsChild::Heading2(builder.build())
    }
}
impl From<Heading3> for DetailsChild {
    fn from(child: Heading3) -> Self {
        DetailsChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for DetailsChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        DetailsChild::Heading3(builder.build())
    }
}
impl From<Heading4> for DetailsChild {
    fn from(child: Heading4) -> Self {
        DetailsChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for DetailsChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        DetailsChild::Heading4(builder.build())
    }
}
impl From<Heading5> for DetailsChild {
    fn from(child: Heading5) -> Self {
        DetailsChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for DetailsChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        DetailsChild::Heading5(builder.build())
    }
}
impl From<Heading6> for DetailsChild {
    fn from(child: Heading6) -> Self {
        DetailsChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for DetailsChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        DetailsChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for DetailsChild {
    fn from(child: HeadingGroup) -> Self {
        DetailsChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for DetailsChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        DetailsChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for DetailsChild {
    fn from(child: HorizontalRule) -> Self {
        DetailsChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for DetailsChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        DetailsChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for DetailsChild {
    fn from(child: Image) -> Self {
        DetailsChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for DetailsChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        DetailsChild::Image(builder.build())
    }
}
impl From<InlineFrame> for DetailsChild {
    fn from(child: InlineFrame) -> Self {
        DetailsChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for DetailsChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        DetailsChild::InlineFrame(builder.build())
    }
}
impl From<Input> for DetailsChild {
    fn from(child: Input) -> Self {
        DetailsChild::Input(child)
    }
}
impl From<builders::InputBuilder> for DetailsChild {
    fn from(builder: builders::InputBuilder) -> Self {
        DetailsChild::Input(builder.build())
    }
}
impl From<Inserted> for DetailsChild {
    fn from(child: Inserted) -> Self {
        DetailsChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for DetailsChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        DetailsChild::Inserted(builder.build())
    }
}
impl From<Italic> for DetailsChild {
    fn from(child: Italic) -> Self {
        DetailsChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for DetailsChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        DetailsChild::Italic(builder.build())
    }
}
impl From<Keyboard> for DetailsChild {
    fn from(child: Keyboard) -> Self {
        DetailsChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for DetailsChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        DetailsChild::Keyboard(builder.build())
    }
}
impl From<Label> for DetailsChild {
    fn from(child: Label) -> Self {
        DetailsChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for DetailsChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        DetailsChild::Label(builder.build())
    }
}
impl From<LineBreak> for DetailsChild {
    fn from(child: LineBreak) -> Self {
        DetailsChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for DetailsChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        DetailsChild::LineBreak(builder.build())
    }
}
impl From<Link> for DetailsChild {
    fn from(child: Link) -> Self {
        DetailsChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for DetailsChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        DetailsChild::Link(builder.build())
    }
}
impl From<Main> for DetailsChild {
    fn from(child: Main) -> Self {
        DetailsChild::Main(child)
    }
}
impl From<builders::MainBuilder> for DetailsChild {
    fn from(builder: builders::MainBuilder) -> Self {
        DetailsChild::Main(builder.build())
    }
}
impl From<Map> for DetailsChild {
    fn from(child: Map) -> Self {
        DetailsChild::Map(child)
    }
}
impl From<builders::MapBuilder> for DetailsChild {
    fn from(builder: builders::MapBuilder) -> Self {
        DetailsChild::Map(builder.build())
    }
}
impl From<MapArea> for DetailsChild {
    fn from(child: MapArea) -> Self {
        DetailsChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for DetailsChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        DetailsChild::MapArea(builder.build())
    }
}
impl From<Mark> for DetailsChild {
    fn from(child: Mark) -> Self {
        DetailsChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for DetailsChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        DetailsChild::Mark(builder.build())
    }
}
impl From<Menu> for DetailsChild {
    fn from(child: Menu) -> Self {
        DetailsChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for DetailsChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        DetailsChild::Menu(builder.build())
    }
}
impl From<Metadata> for DetailsChild {
    fn from(child: Metadata) -> Self {
        DetailsChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for DetailsChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        DetailsChild::Metadata(builder.build())
    }
}
impl From<Meter> for DetailsChild {
    fn from(child: Meter) -> Self {
        DetailsChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for DetailsChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        DetailsChild::Meter(builder.build())
    }
}
impl From<Navigation> for DetailsChild {
    fn from(child: Navigation) -> Self {
        DetailsChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for DetailsChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        DetailsChild::Navigation(builder.build())
    }
}
impl From<NoScript> for DetailsChild {
    fn from(child: NoScript) -> Self {
        DetailsChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for DetailsChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        DetailsChild::NoScript(builder.build())
    }
}
impl From<Object> for DetailsChild {
    fn from(child: Object) -> Self {
        DetailsChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for DetailsChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        DetailsChild::Object(builder.build())
    }
}
impl From<OrderedList> for DetailsChild {
    fn from(child: OrderedList) -> Self {
        DetailsChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for DetailsChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        DetailsChild::OrderedList(builder.build())
    }
}
impl From<Output> for DetailsChild {
    fn from(child: Output) -> Self {
        DetailsChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for DetailsChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        DetailsChild::Output(builder.build())
    }
}
impl From<Paragraph> for DetailsChild {
    fn from(child: Paragraph) -> Self {
        DetailsChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for DetailsChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        DetailsChild::Paragraph(builder.build())
    }
}
impl From<Picture> for DetailsChild {
    fn from(child: Picture) -> Self {
        DetailsChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for DetailsChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        DetailsChild::Picture(builder.build())
    }
}
impl From<Preformatted> for DetailsChild {
    fn from(child: Preformatted) -> Self {
        DetailsChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for DetailsChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        DetailsChild::Preformatted(builder.build())
    }
}
impl From<Progress> for DetailsChild {
    fn from(child: Progress) -> Self {
        DetailsChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for DetailsChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        DetailsChild::Progress(builder.build())
    }
}
impl From<Quote> for DetailsChild {
    fn from(child: Quote) -> Self {
        DetailsChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for DetailsChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        DetailsChild::Quote(builder.build())
    }
}
impl From<Ruby> for DetailsChild {
    fn from(child: Ruby) -> Self {
        DetailsChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for DetailsChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        DetailsChild::Ruby(builder.build())
    }
}
impl From<Sample> for DetailsChild {
    fn from(child: Sample) -> Self {
        DetailsChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for DetailsChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        DetailsChild::Sample(builder.build())
    }
}
impl From<Script> for DetailsChild {
    fn from(child: Script) -> Self {
        DetailsChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for DetailsChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        DetailsChild::Script(builder.build())
    }
}
impl From<Search> for DetailsChild {
    fn from(child: Search) -> Self {
        DetailsChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for DetailsChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        DetailsChild::Search(builder.build())
    }
}
impl From<Section> for DetailsChild {
    fn from(child: Section) -> Self {
        DetailsChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for DetailsChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        DetailsChild::Section(builder.build())
    }
}
impl From<Select> for DetailsChild {
    fn from(child: Select) -> Self {
        DetailsChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for DetailsChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        DetailsChild::Select(builder.build())
    }
}
impl From<Slot> for DetailsChild {
    fn from(child: Slot) -> Self {
        DetailsChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for DetailsChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        DetailsChild::Slot(builder.build())
    }
}
impl From<Small> for DetailsChild {
    fn from(child: Small) -> Self {
        DetailsChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for DetailsChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        DetailsChild::Small(builder.build())
    }
}
impl From<Span> for DetailsChild {
    fn from(child: Span) -> Self {
        DetailsChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for DetailsChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        DetailsChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for DetailsChild {
    fn from(child: StrikeThrough) -> Self {
        DetailsChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for DetailsChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        DetailsChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for DetailsChild {
    fn from(child: Strong) -> Self {
        DetailsChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for DetailsChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        DetailsChild::Strong(builder.build())
    }
}
impl From<SubScript> for DetailsChild {
    fn from(child: SubScript) -> Self {
        DetailsChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for DetailsChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        DetailsChild::SubScript(builder.build())
    }
}
impl From<Summary> for DetailsChild {
    fn from(child: Summary) -> Self {
        DetailsChild::Summary(child)
    }
}
impl From<builders::SummaryBuilder> for DetailsChild {
    fn from(builder: builders::SummaryBuilder) -> Self {
        DetailsChild::Summary(builder.build())
    }
}
impl From<SupScript> for DetailsChild {
    fn from(child: SupScript) -> Self {
        DetailsChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for DetailsChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        DetailsChild::SupScript(builder.build())
    }
}
impl From<Table> for DetailsChild {
    fn from(child: Table) -> Self {
        DetailsChild::Table(child)
    }
}
impl From<builders::TableBuilder> for DetailsChild {
    fn from(builder: builders::TableBuilder) -> Self {
        DetailsChild::Table(builder.build())
    }
}
impl From<Template> for DetailsChild {
    fn from(child: Template) -> Self {
        DetailsChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for DetailsChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        DetailsChild::Template(builder.build())
    }
}
impl From<TextArea> for DetailsChild {
    fn from(child: TextArea) -> Self {
        DetailsChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for DetailsChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        DetailsChild::TextArea(builder.build())
    }
}
impl From<Time> for DetailsChild {
    fn from(child: Time) -> Self {
        DetailsChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for DetailsChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        DetailsChild::Time(builder.build())
    }
}
impl From<Underline> for DetailsChild {
    fn from(child: Underline) -> Self {
        DetailsChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for DetailsChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        DetailsChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for DetailsChild {
    fn from(child: UnorderedList) -> Self {
        DetailsChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for DetailsChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        DetailsChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for DetailsChild {
    fn from(child: Variable) -> Self {
        DetailsChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for DetailsChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        DetailsChild::Variable(builder.build())
    }
}
impl From<Video> for DetailsChild {
    fn from(child: Video) -> Self {
        DetailsChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for DetailsChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        DetailsChild::Video(builder.build())
    }
}
impl From<WordBreak> for DetailsChild {
    fn from(child: WordBreak) -> Self {
        DetailsChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for DetailsChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        DetailsChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for DetailsChild {
    fn from(s: &'static str) -> Self {
        DetailsChild::Text(s.into())
    }
}
impl From<String> for DetailsChild {
    fn from(s: String) -> Self {
        DetailsChild::Text(s.into())
    }
}
impl From<CowStr> for DetailsChild {
    fn from(s: CowStr) -> Self {
        DetailsChild::Text(s)
    }
}
impl std::fmt::Debug for DetailsChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DetailsChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Address(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Article(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Button(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Code(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Data(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Details(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Division(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Form(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Header(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Image(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Input(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Label(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Link(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Main(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Map(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Object(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Output(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Script(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Search(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Section(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Select(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Small(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Span(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Summary(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Table(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Template(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Time(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Video(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            DetailsChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for DetailsChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DetailsChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Address(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Article(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Aside(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Audio(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DetailsChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Bold(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Button(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Cite(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Code(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Custom(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Data(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::DataList(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Definition(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Details(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Division(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Embed(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Figure(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Footer(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Form(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Header(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Image(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Input(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Italic(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Label(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Link(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Main(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Map(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Mark(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Menu(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Meter(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Object(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Output(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Picture(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Progress(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Quote(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Sample(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Script(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Search(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Section(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Select(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Slot(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Small(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Span(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Strong(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Summary(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Table(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Template(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Time(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Underline(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Variable(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Video(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            DetailsChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
