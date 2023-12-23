// 🤖 This file is generated!

use crate::*;
/// The `<canvas>` element's children.
#[derive(Clone)]
pub enum CanvasChild {
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
impl From<Abbreviation> for CanvasChild {
    fn from(child: Abbreviation) -> Self {
        CanvasChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for CanvasChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        CanvasChild::Abbreviation(builder.build())
    }
}
impl From<Address> for CanvasChild {
    fn from(child: Address) -> Self {
        CanvasChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for CanvasChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        CanvasChild::Address(builder.build())
    }
}
impl From<Anchor> for CanvasChild {
    fn from(child: Anchor) -> Self {
        CanvasChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for CanvasChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        CanvasChild::Anchor(builder.build())
    }
}
impl From<Article> for CanvasChild {
    fn from(child: Article) -> Self {
        CanvasChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for CanvasChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        CanvasChild::Article(builder.build())
    }
}
impl From<Aside> for CanvasChild {
    fn from(child: Aside) -> Self {
        CanvasChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for CanvasChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        CanvasChild::Aside(builder.build())
    }
}
impl From<Audio> for CanvasChild {
    fn from(child: Audio) -> Self {
        CanvasChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for CanvasChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        CanvasChild::Audio(builder.build())
    }
}
impl From<Base> for CanvasChild {
    fn from(child: Base) -> Self {
        CanvasChild::Base(child)
    }
}
impl From<builders::BaseBuilder> for CanvasChild {
    fn from(builder: builders::BaseBuilder) -> Self {
        CanvasChild::Base(builder.build())
    }
}
impl From<BidirectionalIsolate> for CanvasChild {
    fn from(child: BidirectionalIsolate) -> Self {
        CanvasChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for CanvasChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        CanvasChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for CanvasChild {
    fn from(child: BidirectionalOverride) -> Self {
        CanvasChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for CanvasChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        CanvasChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for CanvasChild {
    fn from(child: BlockQuote) -> Self {
        CanvasChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for CanvasChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        CanvasChild::BlockQuote(builder.build())
    }
}
impl From<Body> for CanvasChild {
    fn from(child: Body) -> Self {
        CanvasChild::Body(child)
    }
}
impl From<builders::BodyBuilder> for CanvasChild {
    fn from(builder: builders::BodyBuilder) -> Self {
        CanvasChild::Body(builder.build())
    }
}
impl From<Bold> for CanvasChild {
    fn from(child: Bold) -> Self {
        CanvasChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for CanvasChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        CanvasChild::Bold(builder.build())
    }
}
impl From<Button> for CanvasChild {
    fn from(child: Button) -> Self {
        CanvasChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for CanvasChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        CanvasChild::Button(builder.build())
    }
}
impl From<Canvas> for CanvasChild {
    fn from(child: Canvas) -> Self {
        CanvasChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for CanvasChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        CanvasChild::Canvas(builder.build())
    }
}
impl From<Caption> for CanvasChild {
    fn from(child: Caption) -> Self {
        CanvasChild::Caption(child)
    }
}
impl From<builders::CaptionBuilder> for CanvasChild {
    fn from(builder: builders::CaptionBuilder) -> Self {
        CanvasChild::Caption(builder.build())
    }
}
impl From<Cite> for CanvasChild {
    fn from(child: Cite) -> Self {
        CanvasChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for CanvasChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        CanvasChild::Cite(builder.build())
    }
}
impl From<Code> for CanvasChild {
    fn from(child: Code) -> Self {
        CanvasChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for CanvasChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        CanvasChild::Code(builder.build())
    }
}
impl From<Custom> for CanvasChild {
    fn from(child: Custom) -> Self {
        CanvasChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for CanvasChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        CanvasChild::Custom(builder.build())
    }
}
impl From<Data> for CanvasChild {
    fn from(child: Data) -> Self {
        CanvasChild::Data(child)
    }
}
impl From<builders::DataBuilder> for CanvasChild {
    fn from(builder: builders::DataBuilder) -> Self {
        CanvasChild::Data(builder.build())
    }
}
impl From<DataList> for CanvasChild {
    fn from(child: DataList) -> Self {
        CanvasChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for CanvasChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        CanvasChild::DataList(builder.build())
    }
}
impl From<Definition> for CanvasChild {
    fn from(child: Definition) -> Self {
        CanvasChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for CanvasChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        CanvasChild::Definition(builder.build())
    }
}
impl From<Deleted> for CanvasChild {
    fn from(child: Deleted) -> Self {
        CanvasChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for CanvasChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        CanvasChild::Deleted(builder.build())
    }
}
impl From<DescriptionDetails> for CanvasChild {
    fn from(child: DescriptionDetails) -> Self {
        CanvasChild::DescriptionDetails(child)
    }
}
impl From<builders::DescriptionDetailsBuilder> for CanvasChild {
    fn from(builder: builders::DescriptionDetailsBuilder) -> Self {
        CanvasChild::DescriptionDetails(builder.build())
    }
}
impl From<DescriptionList> for CanvasChild {
    fn from(child: DescriptionList) -> Self {
        CanvasChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for CanvasChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        CanvasChild::DescriptionList(builder.build())
    }
}
impl From<DescriptionTerm> for CanvasChild {
    fn from(child: DescriptionTerm) -> Self {
        CanvasChild::DescriptionTerm(child)
    }
}
impl From<builders::DescriptionTermBuilder> for CanvasChild {
    fn from(builder: builders::DescriptionTermBuilder) -> Self {
        CanvasChild::DescriptionTerm(builder.build())
    }
}
impl From<Details> for CanvasChild {
    fn from(child: Details) -> Self {
        CanvasChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for CanvasChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        CanvasChild::Details(builder.build())
    }
}
impl From<Dialog> for CanvasChild {
    fn from(child: Dialog) -> Self {
        CanvasChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for CanvasChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        CanvasChild::Dialog(builder.build())
    }
}
impl From<Division> for CanvasChild {
    fn from(child: Division) -> Self {
        CanvasChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for CanvasChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        CanvasChild::Division(builder.build())
    }
}
impl From<Embed> for CanvasChild {
    fn from(child: Embed) -> Self {
        CanvasChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for CanvasChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        CanvasChild::Embed(builder.build())
    }
}
impl From<Emphasis> for CanvasChild {
    fn from(child: Emphasis) -> Self {
        CanvasChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for CanvasChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        CanvasChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for CanvasChild {
    fn from(child: FieldSet) -> Self {
        CanvasChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for CanvasChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        CanvasChild::FieldSet(builder.build())
    }
}
impl From<FieldSetLegend> for CanvasChild {
    fn from(child: FieldSetLegend) -> Self {
        CanvasChild::FieldSetLegend(child)
    }
}
impl From<builders::FieldSetLegendBuilder> for CanvasChild {
    fn from(builder: builders::FieldSetLegendBuilder) -> Self {
        CanvasChild::FieldSetLegend(builder.build())
    }
}
impl From<Figure> for CanvasChild {
    fn from(child: Figure) -> Self {
        CanvasChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for CanvasChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        CanvasChild::Figure(builder.build())
    }
}
impl From<FigureCaption> for CanvasChild {
    fn from(child: FigureCaption) -> Self {
        CanvasChild::FigureCaption(child)
    }
}
impl From<builders::FigureCaptionBuilder> for CanvasChild {
    fn from(builder: builders::FigureCaptionBuilder) -> Self {
        CanvasChild::FigureCaption(builder.build())
    }
}
impl From<Footer> for CanvasChild {
    fn from(child: Footer) -> Self {
        CanvasChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for CanvasChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        CanvasChild::Footer(builder.build())
    }
}
impl From<Form> for CanvasChild {
    fn from(child: Form) -> Self {
        CanvasChild::Form(child)
    }
}
impl From<builders::FormBuilder> for CanvasChild {
    fn from(builder: builders::FormBuilder) -> Self {
        CanvasChild::Form(builder.build())
    }
}
impl From<Head> for CanvasChild {
    fn from(child: Head) -> Self {
        CanvasChild::Head(child)
    }
}
impl From<builders::HeadBuilder> for CanvasChild {
    fn from(builder: builders::HeadBuilder) -> Self {
        CanvasChild::Head(builder.build())
    }
}
impl From<Header> for CanvasChild {
    fn from(child: Header) -> Self {
        CanvasChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for CanvasChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        CanvasChild::Header(builder.build())
    }
}
impl From<Heading1> for CanvasChild {
    fn from(child: Heading1) -> Self {
        CanvasChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for CanvasChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        CanvasChild::Heading1(builder.build())
    }
}
impl From<Heading2> for CanvasChild {
    fn from(child: Heading2) -> Self {
        CanvasChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for CanvasChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        CanvasChild::Heading2(builder.build())
    }
}
impl From<Heading3> for CanvasChild {
    fn from(child: Heading3) -> Self {
        CanvasChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for CanvasChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        CanvasChild::Heading3(builder.build())
    }
}
impl From<Heading4> for CanvasChild {
    fn from(child: Heading4) -> Self {
        CanvasChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for CanvasChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        CanvasChild::Heading4(builder.build())
    }
}
impl From<Heading5> for CanvasChild {
    fn from(child: Heading5) -> Self {
        CanvasChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for CanvasChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        CanvasChild::Heading5(builder.build())
    }
}
impl From<Heading6> for CanvasChild {
    fn from(child: Heading6) -> Self {
        CanvasChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for CanvasChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        CanvasChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for CanvasChild {
    fn from(child: HeadingGroup) -> Self {
        CanvasChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for CanvasChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        CanvasChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for CanvasChild {
    fn from(child: HorizontalRule) -> Self {
        CanvasChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for CanvasChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        CanvasChild::HorizontalRule(builder.build())
    }
}
impl From<Html> for CanvasChild {
    fn from(child: Html) -> Self {
        CanvasChild::Html(child)
    }
}
impl From<builders::HtmlBuilder> for CanvasChild {
    fn from(builder: builders::HtmlBuilder) -> Self {
        CanvasChild::Html(builder.build())
    }
}
impl From<Image> for CanvasChild {
    fn from(child: Image) -> Self {
        CanvasChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for CanvasChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        CanvasChild::Image(builder.build())
    }
}
impl From<InlineFrame> for CanvasChild {
    fn from(child: InlineFrame) -> Self {
        CanvasChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for CanvasChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        CanvasChild::InlineFrame(builder.build())
    }
}
impl From<Input> for CanvasChild {
    fn from(child: Input) -> Self {
        CanvasChild::Input(child)
    }
}
impl From<builders::InputBuilder> for CanvasChild {
    fn from(builder: builders::InputBuilder) -> Self {
        CanvasChild::Input(builder.build())
    }
}
impl From<Inserted> for CanvasChild {
    fn from(child: Inserted) -> Self {
        CanvasChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for CanvasChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        CanvasChild::Inserted(builder.build())
    }
}
impl From<Italic> for CanvasChild {
    fn from(child: Italic) -> Self {
        CanvasChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for CanvasChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        CanvasChild::Italic(builder.build())
    }
}
impl From<Keyboard> for CanvasChild {
    fn from(child: Keyboard) -> Self {
        CanvasChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for CanvasChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        CanvasChild::Keyboard(builder.build())
    }
}
impl From<Label> for CanvasChild {
    fn from(child: Label) -> Self {
        CanvasChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for CanvasChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        CanvasChild::Label(builder.build())
    }
}
impl From<LineBreak> for CanvasChild {
    fn from(child: LineBreak) -> Self {
        CanvasChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for CanvasChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        CanvasChild::LineBreak(builder.build())
    }
}
impl From<Link> for CanvasChild {
    fn from(child: Link) -> Self {
        CanvasChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for CanvasChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        CanvasChild::Link(builder.build())
    }
}
impl From<ListItem> for CanvasChild {
    fn from(child: ListItem) -> Self {
        CanvasChild::ListItem(child)
    }
}
impl From<builders::ListItemBuilder> for CanvasChild {
    fn from(builder: builders::ListItemBuilder) -> Self {
        CanvasChild::ListItem(builder.build())
    }
}
impl From<Main> for CanvasChild {
    fn from(child: Main) -> Self {
        CanvasChild::Main(child)
    }
}
impl From<builders::MainBuilder> for CanvasChild {
    fn from(builder: builders::MainBuilder) -> Self {
        CanvasChild::Main(builder.build())
    }
}
impl From<Map> for CanvasChild {
    fn from(child: Map) -> Self {
        CanvasChild::Map(child)
    }
}
impl From<builders::MapBuilder> for CanvasChild {
    fn from(builder: builders::MapBuilder) -> Self {
        CanvasChild::Map(builder.build())
    }
}
impl From<MapArea> for CanvasChild {
    fn from(child: MapArea) -> Self {
        CanvasChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for CanvasChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        CanvasChild::MapArea(builder.build())
    }
}
impl From<Mark> for CanvasChild {
    fn from(child: Mark) -> Self {
        CanvasChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for CanvasChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        CanvasChild::Mark(builder.build())
    }
}
impl From<Menu> for CanvasChild {
    fn from(child: Menu) -> Self {
        CanvasChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for CanvasChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        CanvasChild::Menu(builder.build())
    }
}
impl From<Metadata> for CanvasChild {
    fn from(child: Metadata) -> Self {
        CanvasChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for CanvasChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        CanvasChild::Metadata(builder.build())
    }
}
impl From<Meter> for CanvasChild {
    fn from(child: Meter) -> Self {
        CanvasChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for CanvasChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        CanvasChild::Meter(builder.build())
    }
}
impl From<Navigation> for CanvasChild {
    fn from(child: Navigation) -> Self {
        CanvasChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for CanvasChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        CanvasChild::Navigation(builder.build())
    }
}
impl From<NoScript> for CanvasChild {
    fn from(child: NoScript) -> Self {
        CanvasChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for CanvasChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        CanvasChild::NoScript(builder.build())
    }
}
impl From<Object> for CanvasChild {
    fn from(child: Object) -> Self {
        CanvasChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for CanvasChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        CanvasChild::Object(builder.build())
    }
}
impl From<Option> for CanvasChild {
    fn from(child: Option) -> Self {
        CanvasChild::Option(child)
    }
}
impl From<builders::OptionBuilder> for CanvasChild {
    fn from(builder: builders::OptionBuilder) -> Self {
        CanvasChild::Option(builder.build())
    }
}
impl From<OptionGroup> for CanvasChild {
    fn from(child: OptionGroup) -> Self {
        CanvasChild::OptionGroup(child)
    }
}
impl From<builders::OptionGroupBuilder> for CanvasChild {
    fn from(builder: builders::OptionGroupBuilder) -> Self {
        CanvasChild::OptionGroup(builder.build())
    }
}
impl From<OrderedList> for CanvasChild {
    fn from(child: OrderedList) -> Self {
        CanvasChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for CanvasChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        CanvasChild::OrderedList(builder.build())
    }
}
impl From<Output> for CanvasChild {
    fn from(child: Output) -> Self {
        CanvasChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for CanvasChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        CanvasChild::Output(builder.build())
    }
}
impl From<Paragraph> for CanvasChild {
    fn from(child: Paragraph) -> Self {
        CanvasChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for CanvasChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        CanvasChild::Paragraph(builder.build())
    }
}
impl From<Picture> for CanvasChild {
    fn from(child: Picture) -> Self {
        CanvasChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for CanvasChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        CanvasChild::Picture(builder.build())
    }
}
impl From<Preformatted> for CanvasChild {
    fn from(child: Preformatted) -> Self {
        CanvasChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for CanvasChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        CanvasChild::Preformatted(builder.build())
    }
}
impl From<Progress> for CanvasChild {
    fn from(child: Progress) -> Self {
        CanvasChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for CanvasChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        CanvasChild::Progress(builder.build())
    }
}
impl From<Quote> for CanvasChild {
    fn from(child: Quote) -> Self {
        CanvasChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for CanvasChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        CanvasChild::Quote(builder.build())
    }
}
impl From<Ruby> for CanvasChild {
    fn from(child: Ruby) -> Self {
        CanvasChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for CanvasChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        CanvasChild::Ruby(builder.build())
    }
}
impl From<RubyParenthesis> for CanvasChild {
    fn from(child: RubyParenthesis) -> Self {
        CanvasChild::RubyParenthesis(child)
    }
}
impl From<builders::RubyParenthesisBuilder> for CanvasChild {
    fn from(builder: builders::RubyParenthesisBuilder) -> Self {
        CanvasChild::RubyParenthesis(builder.build())
    }
}
impl From<RubyText> for CanvasChild {
    fn from(child: RubyText) -> Self {
        CanvasChild::RubyText(child)
    }
}
impl From<builders::RubyTextBuilder> for CanvasChild {
    fn from(builder: builders::RubyTextBuilder) -> Self {
        CanvasChild::RubyText(builder.build())
    }
}
impl From<Sample> for CanvasChild {
    fn from(child: Sample) -> Self {
        CanvasChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for CanvasChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        CanvasChild::Sample(builder.build())
    }
}
impl From<Script> for CanvasChild {
    fn from(child: Script) -> Self {
        CanvasChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for CanvasChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        CanvasChild::Script(builder.build())
    }
}
impl From<Search> for CanvasChild {
    fn from(child: Search) -> Self {
        CanvasChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for CanvasChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        CanvasChild::Search(builder.build())
    }
}
impl From<Section> for CanvasChild {
    fn from(child: Section) -> Self {
        CanvasChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for CanvasChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        CanvasChild::Section(builder.build())
    }
}
impl From<Select> for CanvasChild {
    fn from(child: Select) -> Self {
        CanvasChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for CanvasChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        CanvasChild::Select(builder.build())
    }
}
impl From<Slot> for CanvasChild {
    fn from(child: Slot) -> Self {
        CanvasChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for CanvasChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        CanvasChild::Slot(builder.build())
    }
}
impl From<Small> for CanvasChild {
    fn from(child: Small) -> Self {
        CanvasChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for CanvasChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        CanvasChild::Small(builder.build())
    }
}
impl From<Source> for CanvasChild {
    fn from(child: Source) -> Self {
        CanvasChild::Source(child)
    }
}
impl From<builders::SourceBuilder> for CanvasChild {
    fn from(builder: builders::SourceBuilder) -> Self {
        CanvasChild::Source(builder.build())
    }
}
impl From<Span> for CanvasChild {
    fn from(child: Span) -> Self {
        CanvasChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for CanvasChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        CanvasChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for CanvasChild {
    fn from(child: StrikeThrough) -> Self {
        CanvasChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for CanvasChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        CanvasChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for CanvasChild {
    fn from(child: Strong) -> Self {
        CanvasChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for CanvasChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        CanvasChild::Strong(builder.build())
    }
}
impl From<Style> for CanvasChild {
    fn from(child: Style) -> Self {
        CanvasChild::Style(child)
    }
}
impl From<builders::StyleBuilder> for CanvasChild {
    fn from(builder: builders::StyleBuilder) -> Self {
        CanvasChild::Style(builder.build())
    }
}
impl From<SubScript> for CanvasChild {
    fn from(child: SubScript) -> Self {
        CanvasChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for CanvasChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        CanvasChild::SubScript(builder.build())
    }
}
impl From<Summary> for CanvasChild {
    fn from(child: Summary) -> Self {
        CanvasChild::Summary(child)
    }
}
impl From<builders::SummaryBuilder> for CanvasChild {
    fn from(builder: builders::SummaryBuilder) -> Self {
        CanvasChild::Summary(builder.build())
    }
}
impl From<SupScript> for CanvasChild {
    fn from(child: SupScript) -> Self {
        CanvasChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for CanvasChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        CanvasChild::SupScript(builder.build())
    }
}
impl From<Table> for CanvasChild {
    fn from(child: Table) -> Self {
        CanvasChild::Table(child)
    }
}
impl From<builders::TableBuilder> for CanvasChild {
    fn from(builder: builders::TableBuilder) -> Self {
        CanvasChild::Table(builder.build())
    }
}
impl From<TableBody> for CanvasChild {
    fn from(child: TableBody) -> Self {
        CanvasChild::TableBody(child)
    }
}
impl From<builders::TableBodyBuilder> for CanvasChild {
    fn from(builder: builders::TableBodyBuilder) -> Self {
        CanvasChild::TableBody(builder.build())
    }
}
impl From<TableCell> for CanvasChild {
    fn from(child: TableCell) -> Self {
        CanvasChild::TableCell(child)
    }
}
impl From<builders::TableCellBuilder> for CanvasChild {
    fn from(builder: builders::TableCellBuilder) -> Self {
        CanvasChild::TableCell(builder.build())
    }
}
impl From<TableColumn> for CanvasChild {
    fn from(child: TableColumn) -> Self {
        CanvasChild::TableColumn(child)
    }
}
impl From<builders::TableColumnBuilder> for CanvasChild {
    fn from(builder: builders::TableColumnBuilder) -> Self {
        CanvasChild::TableColumn(builder.build())
    }
}
impl From<TableColumnGroup> for CanvasChild {
    fn from(child: TableColumnGroup) -> Self {
        CanvasChild::TableColumnGroup(child)
    }
}
impl From<builders::TableColumnGroupBuilder> for CanvasChild {
    fn from(builder: builders::TableColumnGroupBuilder) -> Self {
        CanvasChild::TableColumnGroup(builder.build())
    }
}
impl From<TableFoot> for CanvasChild {
    fn from(child: TableFoot) -> Self {
        CanvasChild::TableFoot(child)
    }
}
impl From<builders::TableFootBuilder> for CanvasChild {
    fn from(builder: builders::TableFootBuilder) -> Self {
        CanvasChild::TableFoot(builder.build())
    }
}
impl From<TableHead> for CanvasChild {
    fn from(child: TableHead) -> Self {
        CanvasChild::TableHead(child)
    }
}
impl From<builders::TableHeadBuilder> for CanvasChild {
    fn from(builder: builders::TableHeadBuilder) -> Self {
        CanvasChild::TableHead(builder.build())
    }
}
impl From<TableHeader> for CanvasChild {
    fn from(child: TableHeader) -> Self {
        CanvasChild::TableHeader(child)
    }
}
impl From<builders::TableHeaderBuilder> for CanvasChild {
    fn from(builder: builders::TableHeaderBuilder) -> Self {
        CanvasChild::TableHeader(builder.build())
    }
}
impl From<TableRow> for CanvasChild {
    fn from(child: TableRow) -> Self {
        CanvasChild::TableRow(child)
    }
}
impl From<builders::TableRowBuilder> for CanvasChild {
    fn from(builder: builders::TableRowBuilder) -> Self {
        CanvasChild::TableRow(builder.build())
    }
}
impl From<Template> for CanvasChild {
    fn from(child: Template) -> Self {
        CanvasChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for CanvasChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        CanvasChild::Template(builder.build())
    }
}
impl From<TextArea> for CanvasChild {
    fn from(child: TextArea) -> Self {
        CanvasChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for CanvasChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        CanvasChild::TextArea(builder.build())
    }
}
impl From<Time> for CanvasChild {
    fn from(child: Time) -> Self {
        CanvasChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for CanvasChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        CanvasChild::Time(builder.build())
    }
}
impl From<Title> for CanvasChild {
    fn from(child: Title) -> Self {
        CanvasChild::Title(child)
    }
}
impl From<builders::TitleBuilder> for CanvasChild {
    fn from(builder: builders::TitleBuilder) -> Self {
        CanvasChild::Title(builder.build())
    }
}
impl From<Track> for CanvasChild {
    fn from(child: Track) -> Self {
        CanvasChild::Track(child)
    }
}
impl From<builders::TrackBuilder> for CanvasChild {
    fn from(builder: builders::TrackBuilder) -> Self {
        CanvasChild::Track(builder.build())
    }
}
impl From<Underline> for CanvasChild {
    fn from(child: Underline) -> Self {
        CanvasChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for CanvasChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        CanvasChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for CanvasChild {
    fn from(child: UnorderedList) -> Self {
        CanvasChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for CanvasChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        CanvasChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for CanvasChild {
    fn from(child: Variable) -> Self {
        CanvasChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for CanvasChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        CanvasChild::Variable(builder.build())
    }
}
impl From<Video> for CanvasChild {
    fn from(child: Video) -> Self {
        CanvasChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for CanvasChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        CanvasChild::Video(builder.build())
    }
}
impl From<WordBreak> for CanvasChild {
    fn from(child: WordBreak) -> Self {
        CanvasChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for CanvasChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        CanvasChild::WordBreak(builder.build())
    }
}
impl std::fmt::Debug for CanvasChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CanvasChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Address(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Article(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Base(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Body(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Button(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Caption(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Code(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Data(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::DescriptionDetails(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::DescriptionTerm(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Details(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Division(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::FieldSetLegend(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::FigureCaption(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Form(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Head(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Header(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Html(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Image(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Input(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Label(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Link(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::ListItem(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Main(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Map(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Object(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Option(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::OptionGroup(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Output(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::RubyParenthesis(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::RubyText(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Script(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Search(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Section(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Select(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Small(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Source(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Span(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Style(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Summary(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Table(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::TableBody(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::TableCell(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::TableColumn(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::TableColumnGroup(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::TableFoot(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::TableHead(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::TableHeader(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::TableRow(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Template(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Time(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Title(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Track(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::Video(child) => std::fmt::Debug::fmt(child, f),
            CanvasChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for CanvasChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CanvasChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Address(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Article(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Aside(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Audio(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Base(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Body(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Bold(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Button(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Caption(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Cite(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Code(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Custom(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Data(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::DataList(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Definition(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::DescriptionDetails(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::DescriptionTerm(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Details(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Division(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Embed(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::FieldSetLegend(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Figure(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::FigureCaption(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Footer(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Form(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Head(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Header(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Html(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Image(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Input(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Italic(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Label(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Link(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::ListItem(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Main(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Map(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Mark(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Menu(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Meter(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Object(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Option(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::OptionGroup(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Output(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Picture(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Progress(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Quote(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::RubyParenthesis(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::RubyText(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Sample(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Script(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Search(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Section(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Select(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Slot(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Small(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Source(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Span(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Strong(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Style(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Summary(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Table(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::TableBody(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::TableCell(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::TableColumn(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::TableColumnGroup(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::TableFoot(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::TableHead(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::TableHeader(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::TableRow(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Template(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Time(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Title(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Track(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Underline(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Variable(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::Video(child) => std::fmt::Display::fmt(child, f),
            CanvasChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
