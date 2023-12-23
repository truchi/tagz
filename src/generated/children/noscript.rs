// 🤖 This file is generated!

use crate::*;
/// The `<noscript>` element's children.
#[derive(Clone)]
pub enum NoScriptChild {
    Abbreviation(Abbreviation),
    Address(Address),
    Anchor(Anchor),
    Article(Article),
    Aside(Aside),
    Audio(Audio),
    Base(Base),
    BidirectionalIsolate(BidirectionalIsolate),
    BidirectionalOverride(BidirectionalOverride),
    BlockQuote(BlockQuote),
    Body(Body),
    Bold(Bold),
    Button(Button),
    Canvas(Canvas),
    Caption(Caption),
    Cite(Cite),
    Code(Code),
    Custom(Custom),
    Data(Data),
    DataList(DataList),
    Definition(Definition),
    Deleted(Deleted),
    DescriptionDetails(DescriptionDetails),
    DescriptionList(DescriptionList),
    DescriptionTerm(DescriptionTerm),
    Details(Details),
    Dialog(Dialog),
    Division(Division),
    Embed(Embed),
    Emphasis(Emphasis),
    FieldSet(FieldSet),
    FieldSetLegend(FieldSetLegend),
    Figure(Figure),
    FigureCaption(FigureCaption),
    Footer(Footer),
    Form(Form),
    Head(Head),
    Header(Header),
    Heading1(Heading1),
    Heading2(Heading2),
    Heading3(Heading3),
    Heading4(Heading4),
    Heading5(Heading5),
    Heading6(Heading6),
    HeadingGroup(HeadingGroup),
    HorizontalRule(HorizontalRule),
    Html(Html),
    Image(Image),
    InlineFrame(InlineFrame),
    Input(Input),
    Inserted(Inserted),
    Italic(Italic),
    Keyboard(Keyboard),
    Label(Label),
    LineBreak(LineBreak),
    Link(Link),
    ListItem(ListItem),
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
    Option(Option),
    OptionGroup(OptionGroup),
    OrderedList(OrderedList),
    Output(Output),
    Paragraph(Paragraph),
    Picture(Picture),
    Preformatted(Preformatted),
    Progress(Progress),
    Quote(Quote),
    Ruby(Ruby),
    RubyParenthesis(RubyParenthesis),
    RubyText(RubyText),
    Sample(Sample),
    Script(Script),
    Search(Search),
    Section(Section),
    Select(Select),
    Slot(Slot),
    Small(Small),
    Source(Source),
    Span(Span),
    StrikeThrough(StrikeThrough),
    Strong(Strong),
    Style(Style),
    SubScript(SubScript),
    Summary(Summary),
    SupScript(SupScript),
    Table(Table),
    TableBody(TableBody),
    TableCell(TableCell),
    TableColumn(TableColumn),
    TableColumnGroup(TableColumnGroup),
    TableFoot(TableFoot),
    TableHead(TableHead),
    TableHeader(TableHeader),
    TableRow(TableRow),
    Template(Template),
    TextArea(TextArea),
    Time(Time),
    Title(Title),
    Track(Track),
    Underline(Underline),
    UnorderedList(UnorderedList),
    Variable(Variable),
    Video(Video),
    WordBreak(WordBreak),
}
impl From<Abbreviation> for NoScriptChild {
    fn from(child: Abbreviation) -> Self {
        NoScriptChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for NoScriptChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        NoScriptChild::Abbreviation(builder.build())
    }
}
impl From<Address> for NoScriptChild {
    fn from(child: Address) -> Self {
        NoScriptChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for NoScriptChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        NoScriptChild::Address(builder.build())
    }
}
impl From<Anchor> for NoScriptChild {
    fn from(child: Anchor) -> Self {
        NoScriptChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for NoScriptChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        NoScriptChild::Anchor(builder.build())
    }
}
impl From<Article> for NoScriptChild {
    fn from(child: Article) -> Self {
        NoScriptChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for NoScriptChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        NoScriptChild::Article(builder.build())
    }
}
impl From<Aside> for NoScriptChild {
    fn from(child: Aside) -> Self {
        NoScriptChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for NoScriptChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        NoScriptChild::Aside(builder.build())
    }
}
impl From<Audio> for NoScriptChild {
    fn from(child: Audio) -> Self {
        NoScriptChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for NoScriptChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        NoScriptChild::Audio(builder.build())
    }
}
impl From<Base> for NoScriptChild {
    fn from(child: Base) -> Self {
        NoScriptChild::Base(child)
    }
}
impl From<builders::BaseBuilder> for NoScriptChild {
    fn from(builder: builders::BaseBuilder) -> Self {
        NoScriptChild::Base(builder.build())
    }
}
impl From<BidirectionalIsolate> for NoScriptChild {
    fn from(child: BidirectionalIsolate) -> Self {
        NoScriptChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for NoScriptChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        NoScriptChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for NoScriptChild {
    fn from(child: BidirectionalOverride) -> Self {
        NoScriptChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for NoScriptChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        NoScriptChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for NoScriptChild {
    fn from(child: BlockQuote) -> Self {
        NoScriptChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for NoScriptChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        NoScriptChild::BlockQuote(builder.build())
    }
}
impl From<Body> for NoScriptChild {
    fn from(child: Body) -> Self {
        NoScriptChild::Body(child)
    }
}
impl From<builders::BodyBuilder> for NoScriptChild {
    fn from(builder: builders::BodyBuilder) -> Self {
        NoScriptChild::Body(builder.build())
    }
}
impl From<Bold> for NoScriptChild {
    fn from(child: Bold) -> Self {
        NoScriptChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for NoScriptChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        NoScriptChild::Bold(builder.build())
    }
}
impl From<Button> for NoScriptChild {
    fn from(child: Button) -> Self {
        NoScriptChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for NoScriptChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        NoScriptChild::Button(builder.build())
    }
}
impl From<Canvas> for NoScriptChild {
    fn from(child: Canvas) -> Self {
        NoScriptChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for NoScriptChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        NoScriptChild::Canvas(builder.build())
    }
}
impl From<Caption> for NoScriptChild {
    fn from(child: Caption) -> Self {
        NoScriptChild::Caption(child)
    }
}
impl From<builders::CaptionBuilder> for NoScriptChild {
    fn from(builder: builders::CaptionBuilder) -> Self {
        NoScriptChild::Caption(builder.build())
    }
}
impl From<Cite> for NoScriptChild {
    fn from(child: Cite) -> Self {
        NoScriptChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for NoScriptChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        NoScriptChild::Cite(builder.build())
    }
}
impl From<Code> for NoScriptChild {
    fn from(child: Code) -> Self {
        NoScriptChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for NoScriptChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        NoScriptChild::Code(builder.build())
    }
}
impl From<Custom> for NoScriptChild {
    fn from(child: Custom) -> Self {
        NoScriptChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for NoScriptChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        NoScriptChild::Custom(builder.build())
    }
}
impl From<Data> for NoScriptChild {
    fn from(child: Data) -> Self {
        NoScriptChild::Data(child)
    }
}
impl From<builders::DataBuilder> for NoScriptChild {
    fn from(builder: builders::DataBuilder) -> Self {
        NoScriptChild::Data(builder.build())
    }
}
impl From<DataList> for NoScriptChild {
    fn from(child: DataList) -> Self {
        NoScriptChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for NoScriptChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        NoScriptChild::DataList(builder.build())
    }
}
impl From<Definition> for NoScriptChild {
    fn from(child: Definition) -> Self {
        NoScriptChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for NoScriptChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        NoScriptChild::Definition(builder.build())
    }
}
impl From<Deleted> for NoScriptChild {
    fn from(child: Deleted) -> Self {
        NoScriptChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for NoScriptChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        NoScriptChild::Deleted(builder.build())
    }
}
impl From<DescriptionDetails> for NoScriptChild {
    fn from(child: DescriptionDetails) -> Self {
        NoScriptChild::DescriptionDetails(child)
    }
}
impl From<builders::DescriptionDetailsBuilder> for NoScriptChild {
    fn from(builder: builders::DescriptionDetailsBuilder) -> Self {
        NoScriptChild::DescriptionDetails(builder.build())
    }
}
impl From<DescriptionList> for NoScriptChild {
    fn from(child: DescriptionList) -> Self {
        NoScriptChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for NoScriptChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        NoScriptChild::DescriptionList(builder.build())
    }
}
impl From<DescriptionTerm> for NoScriptChild {
    fn from(child: DescriptionTerm) -> Self {
        NoScriptChild::DescriptionTerm(child)
    }
}
impl From<builders::DescriptionTermBuilder> for NoScriptChild {
    fn from(builder: builders::DescriptionTermBuilder) -> Self {
        NoScriptChild::DescriptionTerm(builder.build())
    }
}
impl From<Details> for NoScriptChild {
    fn from(child: Details) -> Self {
        NoScriptChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for NoScriptChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        NoScriptChild::Details(builder.build())
    }
}
impl From<Dialog> for NoScriptChild {
    fn from(child: Dialog) -> Self {
        NoScriptChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for NoScriptChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        NoScriptChild::Dialog(builder.build())
    }
}
impl From<Division> for NoScriptChild {
    fn from(child: Division) -> Self {
        NoScriptChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for NoScriptChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        NoScriptChild::Division(builder.build())
    }
}
impl From<Embed> for NoScriptChild {
    fn from(child: Embed) -> Self {
        NoScriptChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for NoScriptChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        NoScriptChild::Embed(builder.build())
    }
}
impl From<Emphasis> for NoScriptChild {
    fn from(child: Emphasis) -> Self {
        NoScriptChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for NoScriptChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        NoScriptChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for NoScriptChild {
    fn from(child: FieldSet) -> Self {
        NoScriptChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for NoScriptChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        NoScriptChild::FieldSet(builder.build())
    }
}
impl From<FieldSetLegend> for NoScriptChild {
    fn from(child: FieldSetLegend) -> Self {
        NoScriptChild::FieldSetLegend(child)
    }
}
impl From<builders::FieldSetLegendBuilder> for NoScriptChild {
    fn from(builder: builders::FieldSetLegendBuilder) -> Self {
        NoScriptChild::FieldSetLegend(builder.build())
    }
}
impl From<Figure> for NoScriptChild {
    fn from(child: Figure) -> Self {
        NoScriptChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for NoScriptChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        NoScriptChild::Figure(builder.build())
    }
}
impl From<FigureCaption> for NoScriptChild {
    fn from(child: FigureCaption) -> Self {
        NoScriptChild::FigureCaption(child)
    }
}
impl From<builders::FigureCaptionBuilder> for NoScriptChild {
    fn from(builder: builders::FigureCaptionBuilder) -> Self {
        NoScriptChild::FigureCaption(builder.build())
    }
}
impl From<Footer> for NoScriptChild {
    fn from(child: Footer) -> Self {
        NoScriptChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for NoScriptChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        NoScriptChild::Footer(builder.build())
    }
}
impl From<Form> for NoScriptChild {
    fn from(child: Form) -> Self {
        NoScriptChild::Form(child)
    }
}
impl From<builders::FormBuilder> for NoScriptChild {
    fn from(builder: builders::FormBuilder) -> Self {
        NoScriptChild::Form(builder.build())
    }
}
impl From<Head> for NoScriptChild {
    fn from(child: Head) -> Self {
        NoScriptChild::Head(child)
    }
}
impl From<builders::HeadBuilder> for NoScriptChild {
    fn from(builder: builders::HeadBuilder) -> Self {
        NoScriptChild::Head(builder.build())
    }
}
impl From<Header> for NoScriptChild {
    fn from(child: Header) -> Self {
        NoScriptChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for NoScriptChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        NoScriptChild::Header(builder.build())
    }
}
impl From<Heading1> for NoScriptChild {
    fn from(child: Heading1) -> Self {
        NoScriptChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for NoScriptChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        NoScriptChild::Heading1(builder.build())
    }
}
impl From<Heading2> for NoScriptChild {
    fn from(child: Heading2) -> Self {
        NoScriptChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for NoScriptChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        NoScriptChild::Heading2(builder.build())
    }
}
impl From<Heading3> for NoScriptChild {
    fn from(child: Heading3) -> Self {
        NoScriptChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for NoScriptChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        NoScriptChild::Heading3(builder.build())
    }
}
impl From<Heading4> for NoScriptChild {
    fn from(child: Heading4) -> Self {
        NoScriptChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for NoScriptChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        NoScriptChild::Heading4(builder.build())
    }
}
impl From<Heading5> for NoScriptChild {
    fn from(child: Heading5) -> Self {
        NoScriptChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for NoScriptChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        NoScriptChild::Heading5(builder.build())
    }
}
impl From<Heading6> for NoScriptChild {
    fn from(child: Heading6) -> Self {
        NoScriptChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for NoScriptChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        NoScriptChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for NoScriptChild {
    fn from(child: HeadingGroup) -> Self {
        NoScriptChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for NoScriptChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        NoScriptChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for NoScriptChild {
    fn from(child: HorizontalRule) -> Self {
        NoScriptChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for NoScriptChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        NoScriptChild::HorizontalRule(builder.build())
    }
}
impl From<Html> for NoScriptChild {
    fn from(child: Html) -> Self {
        NoScriptChild::Html(child)
    }
}
impl From<builders::HtmlBuilder> for NoScriptChild {
    fn from(builder: builders::HtmlBuilder) -> Self {
        NoScriptChild::Html(builder.build())
    }
}
impl From<Image> for NoScriptChild {
    fn from(child: Image) -> Self {
        NoScriptChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for NoScriptChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        NoScriptChild::Image(builder.build())
    }
}
impl From<InlineFrame> for NoScriptChild {
    fn from(child: InlineFrame) -> Self {
        NoScriptChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for NoScriptChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        NoScriptChild::InlineFrame(builder.build())
    }
}
impl From<Input> for NoScriptChild {
    fn from(child: Input) -> Self {
        NoScriptChild::Input(child)
    }
}
impl From<builders::InputBuilder> for NoScriptChild {
    fn from(builder: builders::InputBuilder) -> Self {
        NoScriptChild::Input(builder.build())
    }
}
impl From<Inserted> for NoScriptChild {
    fn from(child: Inserted) -> Self {
        NoScriptChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for NoScriptChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        NoScriptChild::Inserted(builder.build())
    }
}
impl From<Italic> for NoScriptChild {
    fn from(child: Italic) -> Self {
        NoScriptChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for NoScriptChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        NoScriptChild::Italic(builder.build())
    }
}
impl From<Keyboard> for NoScriptChild {
    fn from(child: Keyboard) -> Self {
        NoScriptChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for NoScriptChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        NoScriptChild::Keyboard(builder.build())
    }
}
impl From<Label> for NoScriptChild {
    fn from(child: Label) -> Self {
        NoScriptChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for NoScriptChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        NoScriptChild::Label(builder.build())
    }
}
impl From<LineBreak> for NoScriptChild {
    fn from(child: LineBreak) -> Self {
        NoScriptChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for NoScriptChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        NoScriptChild::LineBreak(builder.build())
    }
}
impl From<Link> for NoScriptChild {
    fn from(child: Link) -> Self {
        NoScriptChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for NoScriptChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        NoScriptChild::Link(builder.build())
    }
}
impl From<ListItem> for NoScriptChild {
    fn from(child: ListItem) -> Self {
        NoScriptChild::ListItem(child)
    }
}
impl From<builders::ListItemBuilder> for NoScriptChild {
    fn from(builder: builders::ListItemBuilder) -> Self {
        NoScriptChild::ListItem(builder.build())
    }
}
impl From<Main> for NoScriptChild {
    fn from(child: Main) -> Self {
        NoScriptChild::Main(child)
    }
}
impl From<builders::MainBuilder> for NoScriptChild {
    fn from(builder: builders::MainBuilder) -> Self {
        NoScriptChild::Main(builder.build())
    }
}
impl From<Map> for NoScriptChild {
    fn from(child: Map) -> Self {
        NoScriptChild::Map(child)
    }
}
impl From<builders::MapBuilder> for NoScriptChild {
    fn from(builder: builders::MapBuilder) -> Self {
        NoScriptChild::Map(builder.build())
    }
}
impl From<MapArea> for NoScriptChild {
    fn from(child: MapArea) -> Self {
        NoScriptChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for NoScriptChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        NoScriptChild::MapArea(builder.build())
    }
}
impl From<Mark> for NoScriptChild {
    fn from(child: Mark) -> Self {
        NoScriptChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for NoScriptChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        NoScriptChild::Mark(builder.build())
    }
}
impl From<Menu> for NoScriptChild {
    fn from(child: Menu) -> Self {
        NoScriptChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for NoScriptChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        NoScriptChild::Menu(builder.build())
    }
}
impl From<Metadata> for NoScriptChild {
    fn from(child: Metadata) -> Self {
        NoScriptChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for NoScriptChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        NoScriptChild::Metadata(builder.build())
    }
}
impl From<Meter> for NoScriptChild {
    fn from(child: Meter) -> Self {
        NoScriptChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for NoScriptChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        NoScriptChild::Meter(builder.build())
    }
}
impl From<Navigation> for NoScriptChild {
    fn from(child: Navigation) -> Self {
        NoScriptChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for NoScriptChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        NoScriptChild::Navigation(builder.build())
    }
}
impl From<NoScript> for NoScriptChild {
    fn from(child: NoScript) -> Self {
        NoScriptChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for NoScriptChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        NoScriptChild::NoScript(builder.build())
    }
}
impl From<Object> for NoScriptChild {
    fn from(child: Object) -> Self {
        NoScriptChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for NoScriptChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        NoScriptChild::Object(builder.build())
    }
}
impl From<Option> for NoScriptChild {
    fn from(child: Option) -> Self {
        NoScriptChild::Option(child)
    }
}
impl From<builders::OptionBuilder> for NoScriptChild {
    fn from(builder: builders::OptionBuilder) -> Self {
        NoScriptChild::Option(builder.build())
    }
}
impl From<OptionGroup> for NoScriptChild {
    fn from(child: OptionGroup) -> Self {
        NoScriptChild::OptionGroup(child)
    }
}
impl From<builders::OptionGroupBuilder> for NoScriptChild {
    fn from(builder: builders::OptionGroupBuilder) -> Self {
        NoScriptChild::OptionGroup(builder.build())
    }
}
impl From<OrderedList> for NoScriptChild {
    fn from(child: OrderedList) -> Self {
        NoScriptChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for NoScriptChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        NoScriptChild::OrderedList(builder.build())
    }
}
impl From<Output> for NoScriptChild {
    fn from(child: Output) -> Self {
        NoScriptChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for NoScriptChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        NoScriptChild::Output(builder.build())
    }
}
impl From<Paragraph> for NoScriptChild {
    fn from(child: Paragraph) -> Self {
        NoScriptChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for NoScriptChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        NoScriptChild::Paragraph(builder.build())
    }
}
impl From<Picture> for NoScriptChild {
    fn from(child: Picture) -> Self {
        NoScriptChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for NoScriptChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        NoScriptChild::Picture(builder.build())
    }
}
impl From<Preformatted> for NoScriptChild {
    fn from(child: Preformatted) -> Self {
        NoScriptChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for NoScriptChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        NoScriptChild::Preformatted(builder.build())
    }
}
impl From<Progress> for NoScriptChild {
    fn from(child: Progress) -> Self {
        NoScriptChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for NoScriptChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        NoScriptChild::Progress(builder.build())
    }
}
impl From<Quote> for NoScriptChild {
    fn from(child: Quote) -> Self {
        NoScriptChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for NoScriptChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        NoScriptChild::Quote(builder.build())
    }
}
impl From<Ruby> for NoScriptChild {
    fn from(child: Ruby) -> Self {
        NoScriptChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for NoScriptChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        NoScriptChild::Ruby(builder.build())
    }
}
impl From<RubyParenthesis> for NoScriptChild {
    fn from(child: RubyParenthesis) -> Self {
        NoScriptChild::RubyParenthesis(child)
    }
}
impl From<builders::RubyParenthesisBuilder> for NoScriptChild {
    fn from(builder: builders::RubyParenthesisBuilder) -> Self {
        NoScriptChild::RubyParenthesis(builder.build())
    }
}
impl From<RubyText> for NoScriptChild {
    fn from(child: RubyText) -> Self {
        NoScriptChild::RubyText(child)
    }
}
impl From<builders::RubyTextBuilder> for NoScriptChild {
    fn from(builder: builders::RubyTextBuilder) -> Self {
        NoScriptChild::RubyText(builder.build())
    }
}
impl From<Sample> for NoScriptChild {
    fn from(child: Sample) -> Self {
        NoScriptChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for NoScriptChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        NoScriptChild::Sample(builder.build())
    }
}
impl From<Script> for NoScriptChild {
    fn from(child: Script) -> Self {
        NoScriptChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for NoScriptChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        NoScriptChild::Script(builder.build())
    }
}
impl From<Search> for NoScriptChild {
    fn from(child: Search) -> Self {
        NoScriptChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for NoScriptChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        NoScriptChild::Search(builder.build())
    }
}
impl From<Section> for NoScriptChild {
    fn from(child: Section) -> Self {
        NoScriptChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for NoScriptChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        NoScriptChild::Section(builder.build())
    }
}
impl From<Select> for NoScriptChild {
    fn from(child: Select) -> Self {
        NoScriptChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for NoScriptChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        NoScriptChild::Select(builder.build())
    }
}
impl From<Slot> for NoScriptChild {
    fn from(child: Slot) -> Self {
        NoScriptChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for NoScriptChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        NoScriptChild::Slot(builder.build())
    }
}
impl From<Small> for NoScriptChild {
    fn from(child: Small) -> Self {
        NoScriptChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for NoScriptChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        NoScriptChild::Small(builder.build())
    }
}
impl From<Source> for NoScriptChild {
    fn from(child: Source) -> Self {
        NoScriptChild::Source(child)
    }
}
impl From<builders::SourceBuilder> for NoScriptChild {
    fn from(builder: builders::SourceBuilder) -> Self {
        NoScriptChild::Source(builder.build())
    }
}
impl From<Span> for NoScriptChild {
    fn from(child: Span) -> Self {
        NoScriptChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for NoScriptChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        NoScriptChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for NoScriptChild {
    fn from(child: StrikeThrough) -> Self {
        NoScriptChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for NoScriptChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        NoScriptChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for NoScriptChild {
    fn from(child: Strong) -> Self {
        NoScriptChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for NoScriptChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        NoScriptChild::Strong(builder.build())
    }
}
impl From<Style> for NoScriptChild {
    fn from(child: Style) -> Self {
        NoScriptChild::Style(child)
    }
}
impl From<builders::StyleBuilder> for NoScriptChild {
    fn from(builder: builders::StyleBuilder) -> Self {
        NoScriptChild::Style(builder.build())
    }
}
impl From<SubScript> for NoScriptChild {
    fn from(child: SubScript) -> Self {
        NoScriptChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for NoScriptChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        NoScriptChild::SubScript(builder.build())
    }
}
impl From<Summary> for NoScriptChild {
    fn from(child: Summary) -> Self {
        NoScriptChild::Summary(child)
    }
}
impl From<builders::SummaryBuilder> for NoScriptChild {
    fn from(builder: builders::SummaryBuilder) -> Self {
        NoScriptChild::Summary(builder.build())
    }
}
impl From<SupScript> for NoScriptChild {
    fn from(child: SupScript) -> Self {
        NoScriptChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for NoScriptChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        NoScriptChild::SupScript(builder.build())
    }
}
impl From<Table> for NoScriptChild {
    fn from(child: Table) -> Self {
        NoScriptChild::Table(child)
    }
}
impl From<builders::TableBuilder> for NoScriptChild {
    fn from(builder: builders::TableBuilder) -> Self {
        NoScriptChild::Table(builder.build())
    }
}
impl From<TableBody> for NoScriptChild {
    fn from(child: TableBody) -> Self {
        NoScriptChild::TableBody(child)
    }
}
impl From<builders::TableBodyBuilder> for NoScriptChild {
    fn from(builder: builders::TableBodyBuilder) -> Self {
        NoScriptChild::TableBody(builder.build())
    }
}
impl From<TableCell> for NoScriptChild {
    fn from(child: TableCell) -> Self {
        NoScriptChild::TableCell(child)
    }
}
impl From<builders::TableCellBuilder> for NoScriptChild {
    fn from(builder: builders::TableCellBuilder) -> Self {
        NoScriptChild::TableCell(builder.build())
    }
}
impl From<TableColumn> for NoScriptChild {
    fn from(child: TableColumn) -> Self {
        NoScriptChild::TableColumn(child)
    }
}
impl From<builders::TableColumnBuilder> for NoScriptChild {
    fn from(builder: builders::TableColumnBuilder) -> Self {
        NoScriptChild::TableColumn(builder.build())
    }
}
impl From<TableColumnGroup> for NoScriptChild {
    fn from(child: TableColumnGroup) -> Self {
        NoScriptChild::TableColumnGroup(child)
    }
}
impl From<builders::TableColumnGroupBuilder> for NoScriptChild {
    fn from(builder: builders::TableColumnGroupBuilder) -> Self {
        NoScriptChild::TableColumnGroup(builder.build())
    }
}
impl From<TableFoot> for NoScriptChild {
    fn from(child: TableFoot) -> Self {
        NoScriptChild::TableFoot(child)
    }
}
impl From<builders::TableFootBuilder> for NoScriptChild {
    fn from(builder: builders::TableFootBuilder) -> Self {
        NoScriptChild::TableFoot(builder.build())
    }
}
impl From<TableHead> for NoScriptChild {
    fn from(child: TableHead) -> Self {
        NoScriptChild::TableHead(child)
    }
}
impl From<builders::TableHeadBuilder> for NoScriptChild {
    fn from(builder: builders::TableHeadBuilder) -> Self {
        NoScriptChild::TableHead(builder.build())
    }
}
impl From<TableHeader> for NoScriptChild {
    fn from(child: TableHeader) -> Self {
        NoScriptChild::TableHeader(child)
    }
}
impl From<builders::TableHeaderBuilder> for NoScriptChild {
    fn from(builder: builders::TableHeaderBuilder) -> Self {
        NoScriptChild::TableHeader(builder.build())
    }
}
impl From<TableRow> for NoScriptChild {
    fn from(child: TableRow) -> Self {
        NoScriptChild::TableRow(child)
    }
}
impl From<builders::TableRowBuilder> for NoScriptChild {
    fn from(builder: builders::TableRowBuilder) -> Self {
        NoScriptChild::TableRow(builder.build())
    }
}
impl From<Template> for NoScriptChild {
    fn from(child: Template) -> Self {
        NoScriptChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for NoScriptChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        NoScriptChild::Template(builder.build())
    }
}
impl From<TextArea> for NoScriptChild {
    fn from(child: TextArea) -> Self {
        NoScriptChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for NoScriptChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        NoScriptChild::TextArea(builder.build())
    }
}
impl From<Time> for NoScriptChild {
    fn from(child: Time) -> Self {
        NoScriptChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for NoScriptChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        NoScriptChild::Time(builder.build())
    }
}
impl From<Title> for NoScriptChild {
    fn from(child: Title) -> Self {
        NoScriptChild::Title(child)
    }
}
impl From<builders::TitleBuilder> for NoScriptChild {
    fn from(builder: builders::TitleBuilder) -> Self {
        NoScriptChild::Title(builder.build())
    }
}
impl From<Track> for NoScriptChild {
    fn from(child: Track) -> Self {
        NoScriptChild::Track(child)
    }
}
impl From<builders::TrackBuilder> for NoScriptChild {
    fn from(builder: builders::TrackBuilder) -> Self {
        NoScriptChild::Track(builder.build())
    }
}
impl From<Underline> for NoScriptChild {
    fn from(child: Underline) -> Self {
        NoScriptChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for NoScriptChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        NoScriptChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for NoScriptChild {
    fn from(child: UnorderedList) -> Self {
        NoScriptChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for NoScriptChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        NoScriptChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for NoScriptChild {
    fn from(child: Variable) -> Self {
        NoScriptChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for NoScriptChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        NoScriptChild::Variable(builder.build())
    }
}
impl From<Video> for NoScriptChild {
    fn from(child: Video) -> Self {
        NoScriptChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for NoScriptChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        NoScriptChild::Video(builder.build())
    }
}
impl From<WordBreak> for NoScriptChild {
    fn from(child: WordBreak) -> Self {
        NoScriptChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for NoScriptChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        NoScriptChild::WordBreak(builder.build())
    }
}
impl std::fmt::Debug for NoScriptChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoScriptChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Address(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Article(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Base(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Body(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Button(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Caption(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Code(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Data(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::DescriptionDetails(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::DescriptionTerm(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Details(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Division(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::FieldSetLegend(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::FigureCaption(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Form(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Head(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Header(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Html(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Image(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Input(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Label(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Link(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::ListItem(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Main(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Map(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Object(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Option(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::OptionGroup(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Output(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::RubyParenthesis(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::RubyText(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Script(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Search(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Section(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Select(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Small(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Source(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Span(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Style(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Summary(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Table(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::TableBody(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::TableCell(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::TableColumn(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::TableColumnGroup(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::TableFoot(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::TableHead(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::TableHeader(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::TableRow(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Template(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Time(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Title(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Track(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::Video(child) => std::fmt::Debug::fmt(child, f),
            NoScriptChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for NoScriptChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoScriptChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Address(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Article(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Aside(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Audio(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Base(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            NoScriptChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            NoScriptChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Body(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Bold(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Button(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Caption(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Cite(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Code(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Custom(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Data(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::DataList(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Definition(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::DescriptionDetails(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::DescriptionTerm(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Details(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Division(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Embed(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::FieldSetLegend(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Figure(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::FigureCaption(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Footer(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Form(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Head(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Header(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Html(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Image(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Input(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Italic(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Label(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Link(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::ListItem(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Main(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Map(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Mark(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Menu(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Meter(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Object(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Option(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::OptionGroup(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Output(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Picture(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Progress(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Quote(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::RubyParenthesis(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::RubyText(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Sample(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Script(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Search(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Section(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Select(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Slot(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Small(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Source(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Span(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Strong(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Style(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Summary(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Table(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::TableBody(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::TableCell(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::TableColumn(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::TableColumnGroup(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::TableFoot(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::TableHead(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::TableHeader(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::TableRow(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Template(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Time(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Title(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Track(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Underline(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Variable(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::Video(child) => std::fmt::Display::fmt(child, f),
            NoScriptChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
