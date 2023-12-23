// 🤖 This file is generated!

use crate::*;
/// The `<a>` element's children.
#[derive(Clone)]
pub enum AnchorChild {
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
impl From<Abbreviation> for AnchorChild {
    fn from(child: Abbreviation) -> Self {
        AnchorChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for AnchorChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        AnchorChild::Abbreviation(builder.build())
    }
}
impl From<Address> for AnchorChild {
    fn from(child: Address) -> Self {
        AnchorChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for AnchorChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        AnchorChild::Address(builder.build())
    }
}
impl From<Anchor> for AnchorChild {
    fn from(child: Anchor) -> Self {
        AnchorChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for AnchorChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        AnchorChild::Anchor(builder.build())
    }
}
impl From<Article> for AnchorChild {
    fn from(child: Article) -> Self {
        AnchorChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for AnchorChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        AnchorChild::Article(builder.build())
    }
}
impl From<Aside> for AnchorChild {
    fn from(child: Aside) -> Self {
        AnchorChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for AnchorChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        AnchorChild::Aside(builder.build())
    }
}
impl From<Audio> for AnchorChild {
    fn from(child: Audio) -> Self {
        AnchorChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for AnchorChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        AnchorChild::Audio(builder.build())
    }
}
impl From<Base> for AnchorChild {
    fn from(child: Base) -> Self {
        AnchorChild::Base(child)
    }
}
impl From<builders::BaseBuilder> for AnchorChild {
    fn from(builder: builders::BaseBuilder) -> Self {
        AnchorChild::Base(builder.build())
    }
}
impl From<BidirectionalIsolate> for AnchorChild {
    fn from(child: BidirectionalIsolate) -> Self {
        AnchorChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for AnchorChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        AnchorChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for AnchorChild {
    fn from(child: BidirectionalOverride) -> Self {
        AnchorChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for AnchorChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        AnchorChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for AnchorChild {
    fn from(child: BlockQuote) -> Self {
        AnchorChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for AnchorChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        AnchorChild::BlockQuote(builder.build())
    }
}
impl From<Body> for AnchorChild {
    fn from(child: Body) -> Self {
        AnchorChild::Body(child)
    }
}
impl From<builders::BodyBuilder> for AnchorChild {
    fn from(builder: builders::BodyBuilder) -> Self {
        AnchorChild::Body(builder.build())
    }
}
impl From<Bold> for AnchorChild {
    fn from(child: Bold) -> Self {
        AnchorChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for AnchorChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        AnchorChild::Bold(builder.build())
    }
}
impl From<Button> for AnchorChild {
    fn from(child: Button) -> Self {
        AnchorChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for AnchorChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        AnchorChild::Button(builder.build())
    }
}
impl From<Canvas> for AnchorChild {
    fn from(child: Canvas) -> Self {
        AnchorChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for AnchorChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        AnchorChild::Canvas(builder.build())
    }
}
impl From<Caption> for AnchorChild {
    fn from(child: Caption) -> Self {
        AnchorChild::Caption(child)
    }
}
impl From<builders::CaptionBuilder> for AnchorChild {
    fn from(builder: builders::CaptionBuilder) -> Self {
        AnchorChild::Caption(builder.build())
    }
}
impl From<Cite> for AnchorChild {
    fn from(child: Cite) -> Self {
        AnchorChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for AnchorChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        AnchorChild::Cite(builder.build())
    }
}
impl From<Code> for AnchorChild {
    fn from(child: Code) -> Self {
        AnchorChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for AnchorChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        AnchorChild::Code(builder.build())
    }
}
impl From<Custom> for AnchorChild {
    fn from(child: Custom) -> Self {
        AnchorChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for AnchorChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        AnchorChild::Custom(builder.build())
    }
}
impl From<Data> for AnchorChild {
    fn from(child: Data) -> Self {
        AnchorChild::Data(child)
    }
}
impl From<builders::DataBuilder> for AnchorChild {
    fn from(builder: builders::DataBuilder) -> Self {
        AnchorChild::Data(builder.build())
    }
}
impl From<DataList> for AnchorChild {
    fn from(child: DataList) -> Self {
        AnchorChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for AnchorChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        AnchorChild::DataList(builder.build())
    }
}
impl From<Definition> for AnchorChild {
    fn from(child: Definition) -> Self {
        AnchorChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for AnchorChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        AnchorChild::Definition(builder.build())
    }
}
impl From<Deleted> for AnchorChild {
    fn from(child: Deleted) -> Self {
        AnchorChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for AnchorChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        AnchorChild::Deleted(builder.build())
    }
}
impl From<DescriptionDetails> for AnchorChild {
    fn from(child: DescriptionDetails) -> Self {
        AnchorChild::DescriptionDetails(child)
    }
}
impl From<builders::DescriptionDetailsBuilder> for AnchorChild {
    fn from(builder: builders::DescriptionDetailsBuilder) -> Self {
        AnchorChild::DescriptionDetails(builder.build())
    }
}
impl From<DescriptionList> for AnchorChild {
    fn from(child: DescriptionList) -> Self {
        AnchorChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for AnchorChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        AnchorChild::DescriptionList(builder.build())
    }
}
impl From<DescriptionTerm> for AnchorChild {
    fn from(child: DescriptionTerm) -> Self {
        AnchorChild::DescriptionTerm(child)
    }
}
impl From<builders::DescriptionTermBuilder> for AnchorChild {
    fn from(builder: builders::DescriptionTermBuilder) -> Self {
        AnchorChild::DescriptionTerm(builder.build())
    }
}
impl From<Details> for AnchorChild {
    fn from(child: Details) -> Self {
        AnchorChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for AnchorChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        AnchorChild::Details(builder.build())
    }
}
impl From<Dialog> for AnchorChild {
    fn from(child: Dialog) -> Self {
        AnchorChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for AnchorChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        AnchorChild::Dialog(builder.build())
    }
}
impl From<Division> for AnchorChild {
    fn from(child: Division) -> Self {
        AnchorChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for AnchorChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        AnchorChild::Division(builder.build())
    }
}
impl From<Embed> for AnchorChild {
    fn from(child: Embed) -> Self {
        AnchorChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for AnchorChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        AnchorChild::Embed(builder.build())
    }
}
impl From<Emphasis> for AnchorChild {
    fn from(child: Emphasis) -> Self {
        AnchorChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for AnchorChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        AnchorChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for AnchorChild {
    fn from(child: FieldSet) -> Self {
        AnchorChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for AnchorChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        AnchorChild::FieldSet(builder.build())
    }
}
impl From<FieldSetLegend> for AnchorChild {
    fn from(child: FieldSetLegend) -> Self {
        AnchorChild::FieldSetLegend(child)
    }
}
impl From<builders::FieldSetLegendBuilder> for AnchorChild {
    fn from(builder: builders::FieldSetLegendBuilder) -> Self {
        AnchorChild::FieldSetLegend(builder.build())
    }
}
impl From<Figure> for AnchorChild {
    fn from(child: Figure) -> Self {
        AnchorChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for AnchorChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        AnchorChild::Figure(builder.build())
    }
}
impl From<FigureCaption> for AnchorChild {
    fn from(child: FigureCaption) -> Self {
        AnchorChild::FigureCaption(child)
    }
}
impl From<builders::FigureCaptionBuilder> for AnchorChild {
    fn from(builder: builders::FigureCaptionBuilder) -> Self {
        AnchorChild::FigureCaption(builder.build())
    }
}
impl From<Footer> for AnchorChild {
    fn from(child: Footer) -> Self {
        AnchorChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for AnchorChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        AnchorChild::Footer(builder.build())
    }
}
impl From<Form> for AnchorChild {
    fn from(child: Form) -> Self {
        AnchorChild::Form(child)
    }
}
impl From<builders::FormBuilder> for AnchorChild {
    fn from(builder: builders::FormBuilder) -> Self {
        AnchorChild::Form(builder.build())
    }
}
impl From<Head> for AnchorChild {
    fn from(child: Head) -> Self {
        AnchorChild::Head(child)
    }
}
impl From<builders::HeadBuilder> for AnchorChild {
    fn from(builder: builders::HeadBuilder) -> Self {
        AnchorChild::Head(builder.build())
    }
}
impl From<Header> for AnchorChild {
    fn from(child: Header) -> Self {
        AnchorChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for AnchorChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        AnchorChild::Header(builder.build())
    }
}
impl From<Heading1> for AnchorChild {
    fn from(child: Heading1) -> Self {
        AnchorChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for AnchorChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        AnchorChild::Heading1(builder.build())
    }
}
impl From<Heading2> for AnchorChild {
    fn from(child: Heading2) -> Self {
        AnchorChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for AnchorChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        AnchorChild::Heading2(builder.build())
    }
}
impl From<Heading3> for AnchorChild {
    fn from(child: Heading3) -> Self {
        AnchorChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for AnchorChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        AnchorChild::Heading3(builder.build())
    }
}
impl From<Heading4> for AnchorChild {
    fn from(child: Heading4) -> Self {
        AnchorChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for AnchorChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        AnchorChild::Heading4(builder.build())
    }
}
impl From<Heading5> for AnchorChild {
    fn from(child: Heading5) -> Self {
        AnchorChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for AnchorChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        AnchorChild::Heading5(builder.build())
    }
}
impl From<Heading6> for AnchorChild {
    fn from(child: Heading6) -> Self {
        AnchorChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for AnchorChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        AnchorChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for AnchorChild {
    fn from(child: HeadingGroup) -> Self {
        AnchorChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for AnchorChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        AnchorChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for AnchorChild {
    fn from(child: HorizontalRule) -> Self {
        AnchorChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for AnchorChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        AnchorChild::HorizontalRule(builder.build())
    }
}
impl From<Html> for AnchorChild {
    fn from(child: Html) -> Self {
        AnchorChild::Html(child)
    }
}
impl From<builders::HtmlBuilder> for AnchorChild {
    fn from(builder: builders::HtmlBuilder) -> Self {
        AnchorChild::Html(builder.build())
    }
}
impl From<Image> for AnchorChild {
    fn from(child: Image) -> Self {
        AnchorChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for AnchorChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        AnchorChild::Image(builder.build())
    }
}
impl From<InlineFrame> for AnchorChild {
    fn from(child: InlineFrame) -> Self {
        AnchorChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for AnchorChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        AnchorChild::InlineFrame(builder.build())
    }
}
impl From<Input> for AnchorChild {
    fn from(child: Input) -> Self {
        AnchorChild::Input(child)
    }
}
impl From<builders::InputBuilder> for AnchorChild {
    fn from(builder: builders::InputBuilder) -> Self {
        AnchorChild::Input(builder.build())
    }
}
impl From<Inserted> for AnchorChild {
    fn from(child: Inserted) -> Self {
        AnchorChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for AnchorChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        AnchorChild::Inserted(builder.build())
    }
}
impl From<Italic> for AnchorChild {
    fn from(child: Italic) -> Self {
        AnchorChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for AnchorChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        AnchorChild::Italic(builder.build())
    }
}
impl From<Keyboard> for AnchorChild {
    fn from(child: Keyboard) -> Self {
        AnchorChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for AnchorChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        AnchorChild::Keyboard(builder.build())
    }
}
impl From<Label> for AnchorChild {
    fn from(child: Label) -> Self {
        AnchorChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for AnchorChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        AnchorChild::Label(builder.build())
    }
}
impl From<LineBreak> for AnchorChild {
    fn from(child: LineBreak) -> Self {
        AnchorChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for AnchorChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        AnchorChild::LineBreak(builder.build())
    }
}
impl From<Link> for AnchorChild {
    fn from(child: Link) -> Self {
        AnchorChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for AnchorChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        AnchorChild::Link(builder.build())
    }
}
impl From<ListItem> for AnchorChild {
    fn from(child: ListItem) -> Self {
        AnchorChild::ListItem(child)
    }
}
impl From<builders::ListItemBuilder> for AnchorChild {
    fn from(builder: builders::ListItemBuilder) -> Self {
        AnchorChild::ListItem(builder.build())
    }
}
impl From<Main> for AnchorChild {
    fn from(child: Main) -> Self {
        AnchorChild::Main(child)
    }
}
impl From<builders::MainBuilder> for AnchorChild {
    fn from(builder: builders::MainBuilder) -> Self {
        AnchorChild::Main(builder.build())
    }
}
impl From<Map> for AnchorChild {
    fn from(child: Map) -> Self {
        AnchorChild::Map(child)
    }
}
impl From<builders::MapBuilder> for AnchorChild {
    fn from(builder: builders::MapBuilder) -> Self {
        AnchorChild::Map(builder.build())
    }
}
impl From<MapArea> for AnchorChild {
    fn from(child: MapArea) -> Self {
        AnchorChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for AnchorChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        AnchorChild::MapArea(builder.build())
    }
}
impl From<Mark> for AnchorChild {
    fn from(child: Mark) -> Self {
        AnchorChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for AnchorChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        AnchorChild::Mark(builder.build())
    }
}
impl From<Menu> for AnchorChild {
    fn from(child: Menu) -> Self {
        AnchorChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for AnchorChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        AnchorChild::Menu(builder.build())
    }
}
impl From<Metadata> for AnchorChild {
    fn from(child: Metadata) -> Self {
        AnchorChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for AnchorChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        AnchorChild::Metadata(builder.build())
    }
}
impl From<Meter> for AnchorChild {
    fn from(child: Meter) -> Self {
        AnchorChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for AnchorChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        AnchorChild::Meter(builder.build())
    }
}
impl From<Navigation> for AnchorChild {
    fn from(child: Navigation) -> Self {
        AnchorChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for AnchorChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        AnchorChild::Navigation(builder.build())
    }
}
impl From<NoScript> for AnchorChild {
    fn from(child: NoScript) -> Self {
        AnchorChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for AnchorChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        AnchorChild::NoScript(builder.build())
    }
}
impl From<Object> for AnchorChild {
    fn from(child: Object) -> Self {
        AnchorChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for AnchorChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        AnchorChild::Object(builder.build())
    }
}
impl From<Option> for AnchorChild {
    fn from(child: Option) -> Self {
        AnchorChild::Option(child)
    }
}
impl From<builders::OptionBuilder> for AnchorChild {
    fn from(builder: builders::OptionBuilder) -> Self {
        AnchorChild::Option(builder.build())
    }
}
impl From<OptionGroup> for AnchorChild {
    fn from(child: OptionGroup) -> Self {
        AnchorChild::OptionGroup(child)
    }
}
impl From<builders::OptionGroupBuilder> for AnchorChild {
    fn from(builder: builders::OptionGroupBuilder) -> Self {
        AnchorChild::OptionGroup(builder.build())
    }
}
impl From<OrderedList> for AnchorChild {
    fn from(child: OrderedList) -> Self {
        AnchorChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for AnchorChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        AnchorChild::OrderedList(builder.build())
    }
}
impl From<Output> for AnchorChild {
    fn from(child: Output) -> Self {
        AnchorChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for AnchorChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        AnchorChild::Output(builder.build())
    }
}
impl From<Paragraph> for AnchorChild {
    fn from(child: Paragraph) -> Self {
        AnchorChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for AnchorChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        AnchorChild::Paragraph(builder.build())
    }
}
impl From<Picture> for AnchorChild {
    fn from(child: Picture) -> Self {
        AnchorChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for AnchorChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        AnchorChild::Picture(builder.build())
    }
}
impl From<Preformatted> for AnchorChild {
    fn from(child: Preformatted) -> Self {
        AnchorChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for AnchorChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        AnchorChild::Preformatted(builder.build())
    }
}
impl From<Progress> for AnchorChild {
    fn from(child: Progress) -> Self {
        AnchorChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for AnchorChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        AnchorChild::Progress(builder.build())
    }
}
impl From<Quote> for AnchorChild {
    fn from(child: Quote) -> Self {
        AnchorChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for AnchorChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        AnchorChild::Quote(builder.build())
    }
}
impl From<Ruby> for AnchorChild {
    fn from(child: Ruby) -> Self {
        AnchorChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for AnchorChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        AnchorChild::Ruby(builder.build())
    }
}
impl From<RubyParenthesis> for AnchorChild {
    fn from(child: RubyParenthesis) -> Self {
        AnchorChild::RubyParenthesis(child)
    }
}
impl From<builders::RubyParenthesisBuilder> for AnchorChild {
    fn from(builder: builders::RubyParenthesisBuilder) -> Self {
        AnchorChild::RubyParenthesis(builder.build())
    }
}
impl From<RubyText> for AnchorChild {
    fn from(child: RubyText) -> Self {
        AnchorChild::RubyText(child)
    }
}
impl From<builders::RubyTextBuilder> for AnchorChild {
    fn from(builder: builders::RubyTextBuilder) -> Self {
        AnchorChild::RubyText(builder.build())
    }
}
impl From<Sample> for AnchorChild {
    fn from(child: Sample) -> Self {
        AnchorChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for AnchorChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        AnchorChild::Sample(builder.build())
    }
}
impl From<Script> for AnchorChild {
    fn from(child: Script) -> Self {
        AnchorChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for AnchorChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        AnchorChild::Script(builder.build())
    }
}
impl From<Search> for AnchorChild {
    fn from(child: Search) -> Self {
        AnchorChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for AnchorChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        AnchorChild::Search(builder.build())
    }
}
impl From<Section> for AnchorChild {
    fn from(child: Section) -> Self {
        AnchorChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for AnchorChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        AnchorChild::Section(builder.build())
    }
}
impl From<Select> for AnchorChild {
    fn from(child: Select) -> Self {
        AnchorChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for AnchorChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        AnchorChild::Select(builder.build())
    }
}
impl From<Slot> for AnchorChild {
    fn from(child: Slot) -> Self {
        AnchorChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for AnchorChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        AnchorChild::Slot(builder.build())
    }
}
impl From<Small> for AnchorChild {
    fn from(child: Small) -> Self {
        AnchorChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for AnchorChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        AnchorChild::Small(builder.build())
    }
}
impl From<Source> for AnchorChild {
    fn from(child: Source) -> Self {
        AnchorChild::Source(child)
    }
}
impl From<builders::SourceBuilder> for AnchorChild {
    fn from(builder: builders::SourceBuilder) -> Self {
        AnchorChild::Source(builder.build())
    }
}
impl From<Span> for AnchorChild {
    fn from(child: Span) -> Self {
        AnchorChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for AnchorChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        AnchorChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for AnchorChild {
    fn from(child: StrikeThrough) -> Self {
        AnchorChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for AnchorChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        AnchorChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for AnchorChild {
    fn from(child: Strong) -> Self {
        AnchorChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for AnchorChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        AnchorChild::Strong(builder.build())
    }
}
impl From<Style> for AnchorChild {
    fn from(child: Style) -> Self {
        AnchorChild::Style(child)
    }
}
impl From<builders::StyleBuilder> for AnchorChild {
    fn from(builder: builders::StyleBuilder) -> Self {
        AnchorChild::Style(builder.build())
    }
}
impl From<SubScript> for AnchorChild {
    fn from(child: SubScript) -> Self {
        AnchorChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for AnchorChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        AnchorChild::SubScript(builder.build())
    }
}
impl From<Summary> for AnchorChild {
    fn from(child: Summary) -> Self {
        AnchorChild::Summary(child)
    }
}
impl From<builders::SummaryBuilder> for AnchorChild {
    fn from(builder: builders::SummaryBuilder) -> Self {
        AnchorChild::Summary(builder.build())
    }
}
impl From<SupScript> for AnchorChild {
    fn from(child: SupScript) -> Self {
        AnchorChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for AnchorChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        AnchorChild::SupScript(builder.build())
    }
}
impl From<Table> for AnchorChild {
    fn from(child: Table) -> Self {
        AnchorChild::Table(child)
    }
}
impl From<builders::TableBuilder> for AnchorChild {
    fn from(builder: builders::TableBuilder) -> Self {
        AnchorChild::Table(builder.build())
    }
}
impl From<TableBody> for AnchorChild {
    fn from(child: TableBody) -> Self {
        AnchorChild::TableBody(child)
    }
}
impl From<builders::TableBodyBuilder> for AnchorChild {
    fn from(builder: builders::TableBodyBuilder) -> Self {
        AnchorChild::TableBody(builder.build())
    }
}
impl From<TableCell> for AnchorChild {
    fn from(child: TableCell) -> Self {
        AnchorChild::TableCell(child)
    }
}
impl From<builders::TableCellBuilder> for AnchorChild {
    fn from(builder: builders::TableCellBuilder) -> Self {
        AnchorChild::TableCell(builder.build())
    }
}
impl From<TableColumn> for AnchorChild {
    fn from(child: TableColumn) -> Self {
        AnchorChild::TableColumn(child)
    }
}
impl From<builders::TableColumnBuilder> for AnchorChild {
    fn from(builder: builders::TableColumnBuilder) -> Self {
        AnchorChild::TableColumn(builder.build())
    }
}
impl From<TableColumnGroup> for AnchorChild {
    fn from(child: TableColumnGroup) -> Self {
        AnchorChild::TableColumnGroup(child)
    }
}
impl From<builders::TableColumnGroupBuilder> for AnchorChild {
    fn from(builder: builders::TableColumnGroupBuilder) -> Self {
        AnchorChild::TableColumnGroup(builder.build())
    }
}
impl From<TableFoot> for AnchorChild {
    fn from(child: TableFoot) -> Self {
        AnchorChild::TableFoot(child)
    }
}
impl From<builders::TableFootBuilder> for AnchorChild {
    fn from(builder: builders::TableFootBuilder) -> Self {
        AnchorChild::TableFoot(builder.build())
    }
}
impl From<TableHead> for AnchorChild {
    fn from(child: TableHead) -> Self {
        AnchorChild::TableHead(child)
    }
}
impl From<builders::TableHeadBuilder> for AnchorChild {
    fn from(builder: builders::TableHeadBuilder) -> Self {
        AnchorChild::TableHead(builder.build())
    }
}
impl From<TableHeader> for AnchorChild {
    fn from(child: TableHeader) -> Self {
        AnchorChild::TableHeader(child)
    }
}
impl From<builders::TableHeaderBuilder> for AnchorChild {
    fn from(builder: builders::TableHeaderBuilder) -> Self {
        AnchorChild::TableHeader(builder.build())
    }
}
impl From<TableRow> for AnchorChild {
    fn from(child: TableRow) -> Self {
        AnchorChild::TableRow(child)
    }
}
impl From<builders::TableRowBuilder> for AnchorChild {
    fn from(builder: builders::TableRowBuilder) -> Self {
        AnchorChild::TableRow(builder.build())
    }
}
impl From<Template> for AnchorChild {
    fn from(child: Template) -> Self {
        AnchorChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for AnchorChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        AnchorChild::Template(builder.build())
    }
}
impl From<TextArea> for AnchorChild {
    fn from(child: TextArea) -> Self {
        AnchorChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for AnchorChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        AnchorChild::TextArea(builder.build())
    }
}
impl From<Time> for AnchorChild {
    fn from(child: Time) -> Self {
        AnchorChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for AnchorChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        AnchorChild::Time(builder.build())
    }
}
impl From<Title> for AnchorChild {
    fn from(child: Title) -> Self {
        AnchorChild::Title(child)
    }
}
impl From<builders::TitleBuilder> for AnchorChild {
    fn from(builder: builders::TitleBuilder) -> Self {
        AnchorChild::Title(builder.build())
    }
}
impl From<Track> for AnchorChild {
    fn from(child: Track) -> Self {
        AnchorChild::Track(child)
    }
}
impl From<builders::TrackBuilder> for AnchorChild {
    fn from(builder: builders::TrackBuilder) -> Self {
        AnchorChild::Track(builder.build())
    }
}
impl From<Underline> for AnchorChild {
    fn from(child: Underline) -> Self {
        AnchorChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for AnchorChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        AnchorChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for AnchorChild {
    fn from(child: UnorderedList) -> Self {
        AnchorChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for AnchorChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        AnchorChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for AnchorChild {
    fn from(child: Variable) -> Self {
        AnchorChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for AnchorChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        AnchorChild::Variable(builder.build())
    }
}
impl From<Video> for AnchorChild {
    fn from(child: Video) -> Self {
        AnchorChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for AnchorChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        AnchorChild::Video(builder.build())
    }
}
impl From<WordBreak> for AnchorChild {
    fn from(child: WordBreak) -> Self {
        AnchorChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for AnchorChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        AnchorChild::WordBreak(builder.build())
    }
}
impl std::fmt::Debug for AnchorChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnchorChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Address(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Article(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Base(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Body(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Button(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Caption(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Code(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Data(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::DescriptionDetails(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::DescriptionTerm(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Details(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Division(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::FieldSetLegend(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::FigureCaption(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Form(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Head(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Header(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Html(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Image(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Input(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Label(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Link(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::ListItem(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Main(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Map(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Object(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Option(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::OptionGroup(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Output(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::RubyParenthesis(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::RubyText(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Script(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Search(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Section(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Select(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Small(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Source(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Span(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Style(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Summary(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Table(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::TableBody(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::TableCell(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::TableColumn(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::TableColumnGroup(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::TableFoot(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::TableHead(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::TableHeader(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::TableRow(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Template(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Time(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Title(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Track(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::Video(child) => std::fmt::Debug::fmt(child, f),
            AnchorChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for AnchorChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnchorChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Address(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Article(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Aside(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Audio(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Base(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Body(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Bold(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Button(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Caption(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Cite(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Code(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Custom(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Data(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::DataList(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Definition(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::DescriptionDetails(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::DescriptionTerm(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Details(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Division(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Embed(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::FieldSetLegend(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Figure(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::FigureCaption(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Footer(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Form(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Head(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Header(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Html(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Image(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Input(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Italic(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Label(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Link(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::ListItem(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Main(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Map(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Mark(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Menu(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Meter(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Object(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Option(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::OptionGroup(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Output(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Picture(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Progress(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Quote(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::RubyParenthesis(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::RubyText(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Sample(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Script(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Search(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Section(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Select(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Slot(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Small(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Source(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Span(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Strong(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Style(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Summary(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Table(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::TableBody(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::TableCell(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::TableColumn(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::TableColumnGroup(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::TableFoot(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::TableHead(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::TableHeader(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::TableRow(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Template(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Time(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Title(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Track(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Underline(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Variable(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::Video(child) => std::fmt::Display::fmt(child, f),
            AnchorChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
