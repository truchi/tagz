// 🤖 This file is generated!

use crate::*;
/// The `<figure>` element's children.
#[derive(Clone)]
pub enum FigureChild {
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
    FigureCaption(FigureCaption),
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
impl From<Abbreviation> for FigureChild {
    fn from(child: Abbreviation) -> Self {
        FigureChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for FigureChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        FigureChild::Abbreviation(builder.build())
    }
}
impl From<Address> for FigureChild {
    fn from(child: Address) -> Self {
        FigureChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for FigureChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        FigureChild::Address(builder.build())
    }
}
impl From<Anchor> for FigureChild {
    fn from(child: Anchor) -> Self {
        FigureChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for FigureChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        FigureChild::Anchor(builder.build())
    }
}
impl From<Article> for FigureChild {
    fn from(child: Article) -> Self {
        FigureChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for FigureChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        FigureChild::Article(builder.build())
    }
}
impl From<Aside> for FigureChild {
    fn from(child: Aside) -> Self {
        FigureChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for FigureChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        FigureChild::Aside(builder.build())
    }
}
impl From<Audio> for FigureChild {
    fn from(child: Audio) -> Self {
        FigureChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for FigureChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        FigureChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for FigureChild {
    fn from(child: BidirectionalIsolate) -> Self {
        FigureChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for FigureChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        FigureChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for FigureChild {
    fn from(child: BidirectionalOverride) -> Self {
        FigureChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for FigureChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        FigureChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for FigureChild {
    fn from(child: BlockQuote) -> Self {
        FigureChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for FigureChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        FigureChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for FigureChild {
    fn from(child: Bold) -> Self {
        FigureChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for FigureChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        FigureChild::Bold(builder.build())
    }
}
impl From<Button> for FigureChild {
    fn from(child: Button) -> Self {
        FigureChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for FigureChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        FigureChild::Button(builder.build())
    }
}
impl From<Canvas> for FigureChild {
    fn from(child: Canvas) -> Self {
        FigureChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for FigureChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        FigureChild::Canvas(builder.build())
    }
}
impl From<Cite> for FigureChild {
    fn from(child: Cite) -> Self {
        FigureChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for FigureChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        FigureChild::Cite(builder.build())
    }
}
impl From<Code> for FigureChild {
    fn from(child: Code) -> Self {
        FigureChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for FigureChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        FigureChild::Code(builder.build())
    }
}
impl From<Custom> for FigureChild {
    fn from(child: Custom) -> Self {
        FigureChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for FigureChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        FigureChild::Custom(builder.build())
    }
}
impl From<Data> for FigureChild {
    fn from(child: Data) -> Self {
        FigureChild::Data(child)
    }
}
impl From<builders::DataBuilder> for FigureChild {
    fn from(builder: builders::DataBuilder) -> Self {
        FigureChild::Data(builder.build())
    }
}
impl From<DataList> for FigureChild {
    fn from(child: DataList) -> Self {
        FigureChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for FigureChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        FigureChild::DataList(builder.build())
    }
}
impl From<Definition> for FigureChild {
    fn from(child: Definition) -> Self {
        FigureChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for FigureChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        FigureChild::Definition(builder.build())
    }
}
impl From<Deleted> for FigureChild {
    fn from(child: Deleted) -> Self {
        FigureChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for FigureChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        FigureChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for FigureChild {
    fn from(child: DescriptionList) -> Self {
        FigureChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for FigureChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        FigureChild::DescriptionList(builder.build())
    }
}
impl From<Details> for FigureChild {
    fn from(child: Details) -> Self {
        FigureChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for FigureChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        FigureChild::Details(builder.build())
    }
}
impl From<Dialog> for FigureChild {
    fn from(child: Dialog) -> Self {
        FigureChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for FigureChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        FigureChild::Dialog(builder.build())
    }
}
impl From<Division> for FigureChild {
    fn from(child: Division) -> Self {
        FigureChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for FigureChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        FigureChild::Division(builder.build())
    }
}
impl From<Embed> for FigureChild {
    fn from(child: Embed) -> Self {
        FigureChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for FigureChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        FigureChild::Embed(builder.build())
    }
}
impl From<Emphasis> for FigureChild {
    fn from(child: Emphasis) -> Self {
        FigureChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for FigureChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        FigureChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for FigureChild {
    fn from(child: FieldSet) -> Self {
        FigureChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for FigureChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        FigureChild::FieldSet(builder.build())
    }
}
impl From<Figure> for FigureChild {
    fn from(child: Figure) -> Self {
        FigureChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for FigureChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        FigureChild::Figure(builder.build())
    }
}
impl From<FigureCaption> for FigureChild {
    fn from(child: FigureCaption) -> Self {
        FigureChild::FigureCaption(child)
    }
}
impl From<builders::FigureCaptionBuilder> for FigureChild {
    fn from(builder: builders::FigureCaptionBuilder) -> Self {
        FigureChild::FigureCaption(builder.build())
    }
}
impl From<Footer> for FigureChild {
    fn from(child: Footer) -> Self {
        FigureChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for FigureChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        FigureChild::Footer(builder.build())
    }
}
impl From<Form> for FigureChild {
    fn from(child: Form) -> Self {
        FigureChild::Form(child)
    }
}
impl From<builders::FormBuilder> for FigureChild {
    fn from(builder: builders::FormBuilder) -> Self {
        FigureChild::Form(builder.build())
    }
}
impl From<Header> for FigureChild {
    fn from(child: Header) -> Self {
        FigureChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for FigureChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        FigureChild::Header(builder.build())
    }
}
impl From<Heading1> for FigureChild {
    fn from(child: Heading1) -> Self {
        FigureChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for FigureChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        FigureChild::Heading1(builder.build())
    }
}
impl From<Heading2> for FigureChild {
    fn from(child: Heading2) -> Self {
        FigureChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for FigureChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        FigureChild::Heading2(builder.build())
    }
}
impl From<Heading3> for FigureChild {
    fn from(child: Heading3) -> Self {
        FigureChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for FigureChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        FigureChild::Heading3(builder.build())
    }
}
impl From<Heading4> for FigureChild {
    fn from(child: Heading4) -> Self {
        FigureChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for FigureChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        FigureChild::Heading4(builder.build())
    }
}
impl From<Heading5> for FigureChild {
    fn from(child: Heading5) -> Self {
        FigureChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for FigureChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        FigureChild::Heading5(builder.build())
    }
}
impl From<Heading6> for FigureChild {
    fn from(child: Heading6) -> Self {
        FigureChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for FigureChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        FigureChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for FigureChild {
    fn from(child: HeadingGroup) -> Self {
        FigureChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for FigureChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        FigureChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for FigureChild {
    fn from(child: HorizontalRule) -> Self {
        FigureChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for FigureChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        FigureChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for FigureChild {
    fn from(child: Image) -> Self {
        FigureChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for FigureChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        FigureChild::Image(builder.build())
    }
}
impl From<InlineFrame> for FigureChild {
    fn from(child: InlineFrame) -> Self {
        FigureChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for FigureChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        FigureChild::InlineFrame(builder.build())
    }
}
impl From<Input> for FigureChild {
    fn from(child: Input) -> Self {
        FigureChild::Input(child)
    }
}
impl From<builders::InputBuilder> for FigureChild {
    fn from(builder: builders::InputBuilder) -> Self {
        FigureChild::Input(builder.build())
    }
}
impl From<Inserted> for FigureChild {
    fn from(child: Inserted) -> Self {
        FigureChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for FigureChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        FigureChild::Inserted(builder.build())
    }
}
impl From<Italic> for FigureChild {
    fn from(child: Italic) -> Self {
        FigureChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for FigureChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        FigureChild::Italic(builder.build())
    }
}
impl From<Keyboard> for FigureChild {
    fn from(child: Keyboard) -> Self {
        FigureChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for FigureChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        FigureChild::Keyboard(builder.build())
    }
}
impl From<Label> for FigureChild {
    fn from(child: Label) -> Self {
        FigureChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for FigureChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        FigureChild::Label(builder.build())
    }
}
impl From<LineBreak> for FigureChild {
    fn from(child: LineBreak) -> Self {
        FigureChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for FigureChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        FigureChild::LineBreak(builder.build())
    }
}
impl From<Link> for FigureChild {
    fn from(child: Link) -> Self {
        FigureChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for FigureChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        FigureChild::Link(builder.build())
    }
}
impl From<Main> for FigureChild {
    fn from(child: Main) -> Self {
        FigureChild::Main(child)
    }
}
impl From<builders::MainBuilder> for FigureChild {
    fn from(builder: builders::MainBuilder) -> Self {
        FigureChild::Main(builder.build())
    }
}
impl From<Map> for FigureChild {
    fn from(child: Map) -> Self {
        FigureChild::Map(child)
    }
}
impl From<builders::MapBuilder> for FigureChild {
    fn from(builder: builders::MapBuilder) -> Self {
        FigureChild::Map(builder.build())
    }
}
impl From<MapArea> for FigureChild {
    fn from(child: MapArea) -> Self {
        FigureChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for FigureChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        FigureChild::MapArea(builder.build())
    }
}
impl From<Mark> for FigureChild {
    fn from(child: Mark) -> Self {
        FigureChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for FigureChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        FigureChild::Mark(builder.build())
    }
}
impl From<Menu> for FigureChild {
    fn from(child: Menu) -> Self {
        FigureChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for FigureChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        FigureChild::Menu(builder.build())
    }
}
impl From<Metadata> for FigureChild {
    fn from(child: Metadata) -> Self {
        FigureChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for FigureChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        FigureChild::Metadata(builder.build())
    }
}
impl From<Meter> for FigureChild {
    fn from(child: Meter) -> Self {
        FigureChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for FigureChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        FigureChild::Meter(builder.build())
    }
}
impl From<Navigation> for FigureChild {
    fn from(child: Navigation) -> Self {
        FigureChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for FigureChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        FigureChild::Navigation(builder.build())
    }
}
impl From<NoScript> for FigureChild {
    fn from(child: NoScript) -> Self {
        FigureChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for FigureChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        FigureChild::NoScript(builder.build())
    }
}
impl From<Object> for FigureChild {
    fn from(child: Object) -> Self {
        FigureChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for FigureChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        FigureChild::Object(builder.build())
    }
}
impl From<OrderedList> for FigureChild {
    fn from(child: OrderedList) -> Self {
        FigureChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for FigureChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        FigureChild::OrderedList(builder.build())
    }
}
impl From<Output> for FigureChild {
    fn from(child: Output) -> Self {
        FigureChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for FigureChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        FigureChild::Output(builder.build())
    }
}
impl From<Paragraph> for FigureChild {
    fn from(child: Paragraph) -> Self {
        FigureChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for FigureChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        FigureChild::Paragraph(builder.build())
    }
}
impl From<Picture> for FigureChild {
    fn from(child: Picture) -> Self {
        FigureChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for FigureChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        FigureChild::Picture(builder.build())
    }
}
impl From<Preformatted> for FigureChild {
    fn from(child: Preformatted) -> Self {
        FigureChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for FigureChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        FigureChild::Preformatted(builder.build())
    }
}
impl From<Progress> for FigureChild {
    fn from(child: Progress) -> Self {
        FigureChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for FigureChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        FigureChild::Progress(builder.build())
    }
}
impl From<Quote> for FigureChild {
    fn from(child: Quote) -> Self {
        FigureChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for FigureChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        FigureChild::Quote(builder.build())
    }
}
impl From<Ruby> for FigureChild {
    fn from(child: Ruby) -> Self {
        FigureChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for FigureChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        FigureChild::Ruby(builder.build())
    }
}
impl From<Sample> for FigureChild {
    fn from(child: Sample) -> Self {
        FigureChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for FigureChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        FigureChild::Sample(builder.build())
    }
}
impl From<Script> for FigureChild {
    fn from(child: Script) -> Self {
        FigureChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for FigureChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        FigureChild::Script(builder.build())
    }
}
impl From<Search> for FigureChild {
    fn from(child: Search) -> Self {
        FigureChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for FigureChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        FigureChild::Search(builder.build())
    }
}
impl From<Section> for FigureChild {
    fn from(child: Section) -> Self {
        FigureChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for FigureChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        FigureChild::Section(builder.build())
    }
}
impl From<Select> for FigureChild {
    fn from(child: Select) -> Self {
        FigureChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for FigureChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        FigureChild::Select(builder.build())
    }
}
impl From<Slot> for FigureChild {
    fn from(child: Slot) -> Self {
        FigureChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for FigureChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        FigureChild::Slot(builder.build())
    }
}
impl From<Small> for FigureChild {
    fn from(child: Small) -> Self {
        FigureChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for FigureChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        FigureChild::Small(builder.build())
    }
}
impl From<Span> for FigureChild {
    fn from(child: Span) -> Self {
        FigureChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for FigureChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        FigureChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for FigureChild {
    fn from(child: StrikeThrough) -> Self {
        FigureChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for FigureChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        FigureChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for FigureChild {
    fn from(child: Strong) -> Self {
        FigureChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for FigureChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        FigureChild::Strong(builder.build())
    }
}
impl From<SubScript> for FigureChild {
    fn from(child: SubScript) -> Self {
        FigureChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for FigureChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        FigureChild::SubScript(builder.build())
    }
}
impl From<SupScript> for FigureChild {
    fn from(child: SupScript) -> Self {
        FigureChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for FigureChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        FigureChild::SupScript(builder.build())
    }
}
impl From<Table> for FigureChild {
    fn from(child: Table) -> Self {
        FigureChild::Table(child)
    }
}
impl From<builders::TableBuilder> for FigureChild {
    fn from(builder: builders::TableBuilder) -> Self {
        FigureChild::Table(builder.build())
    }
}
impl From<Template> for FigureChild {
    fn from(child: Template) -> Self {
        FigureChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for FigureChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        FigureChild::Template(builder.build())
    }
}
impl From<TextArea> for FigureChild {
    fn from(child: TextArea) -> Self {
        FigureChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for FigureChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        FigureChild::TextArea(builder.build())
    }
}
impl From<Time> for FigureChild {
    fn from(child: Time) -> Self {
        FigureChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for FigureChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        FigureChild::Time(builder.build())
    }
}
impl From<Underline> for FigureChild {
    fn from(child: Underline) -> Self {
        FigureChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for FigureChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        FigureChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for FigureChild {
    fn from(child: UnorderedList) -> Self {
        FigureChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for FigureChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        FigureChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for FigureChild {
    fn from(child: Variable) -> Self {
        FigureChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for FigureChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        FigureChild::Variable(builder.build())
    }
}
impl From<Video> for FigureChild {
    fn from(child: Video) -> Self {
        FigureChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for FigureChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        FigureChild::Video(builder.build())
    }
}
impl From<WordBreak> for FigureChild {
    fn from(child: WordBreak) -> Self {
        FigureChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for FigureChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        FigureChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for FigureChild {
    fn from(s: &'static str) -> Self {
        FigureChild::Text(s.into())
    }
}
impl From<String> for FigureChild {
    fn from(s: String) -> Self {
        FigureChild::Text(s.into())
    }
}
impl From<CowStr> for FigureChild {
    fn from(s: CowStr) -> Self {
        FigureChild::Text(s)
    }
}
impl std::fmt::Debug for FigureChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FigureChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Address(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Article(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Button(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Code(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Data(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Details(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Division(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::FigureCaption(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Form(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Header(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Image(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Input(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Label(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Link(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Main(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Map(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Object(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Output(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Script(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Search(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Section(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Select(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Small(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Span(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Table(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Template(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Time(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Video(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            FigureChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for FigureChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FigureChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Address(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Article(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Aside(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Audio(child) => std::fmt::Display::fmt(child, f),
            FigureChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            FigureChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            FigureChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Bold(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Button(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Cite(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Code(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Custom(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Data(child) => std::fmt::Display::fmt(child, f),
            FigureChild::DataList(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Definition(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            FigureChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Details(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Division(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Embed(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            FigureChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Figure(child) => std::fmt::Display::fmt(child, f),
            FigureChild::FigureCaption(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Footer(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Form(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Header(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            FigureChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            FigureChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Image(child) => std::fmt::Display::fmt(child, f),
            FigureChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Input(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Italic(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Label(child) => std::fmt::Display::fmt(child, f),
            FigureChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Link(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Main(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Map(child) => std::fmt::Display::fmt(child, f),
            FigureChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Mark(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Menu(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Meter(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            FigureChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Object(child) => std::fmt::Display::fmt(child, f),
            FigureChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Output(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Picture(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Progress(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Quote(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Sample(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Script(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Search(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Section(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Select(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Slot(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Small(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Span(child) => std::fmt::Display::fmt(child, f),
            FigureChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Strong(child) => std::fmt::Display::fmt(child, f),
            FigureChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            FigureChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Table(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Template(child) => std::fmt::Display::fmt(child, f),
            FigureChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Time(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Underline(child) => std::fmt::Display::fmt(child, f),
            FigureChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Variable(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Video(child) => std::fmt::Display::fmt(child, f),
            FigureChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            FigureChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
