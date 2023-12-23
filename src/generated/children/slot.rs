// 🤖 This file is generated!

use crate::*;
/// The `<slot>` element's children.
#[derive(Clone)]
pub enum SlotChild {
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
impl From<Abbreviation> for SlotChild {
    fn from(child: Abbreviation) -> Self {
        SlotChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for SlotChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        SlotChild::Abbreviation(builder.build())
    }
}
impl From<Address> for SlotChild {
    fn from(child: Address) -> Self {
        SlotChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for SlotChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        SlotChild::Address(builder.build())
    }
}
impl From<Anchor> for SlotChild {
    fn from(child: Anchor) -> Self {
        SlotChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for SlotChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        SlotChild::Anchor(builder.build())
    }
}
impl From<Article> for SlotChild {
    fn from(child: Article) -> Self {
        SlotChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for SlotChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        SlotChild::Article(builder.build())
    }
}
impl From<Aside> for SlotChild {
    fn from(child: Aside) -> Self {
        SlotChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for SlotChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        SlotChild::Aside(builder.build())
    }
}
impl From<Audio> for SlotChild {
    fn from(child: Audio) -> Self {
        SlotChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for SlotChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        SlotChild::Audio(builder.build())
    }
}
impl From<Base> for SlotChild {
    fn from(child: Base) -> Self {
        SlotChild::Base(child)
    }
}
impl From<builders::BaseBuilder> for SlotChild {
    fn from(builder: builders::BaseBuilder) -> Self {
        SlotChild::Base(builder.build())
    }
}
impl From<BidirectionalIsolate> for SlotChild {
    fn from(child: BidirectionalIsolate) -> Self {
        SlotChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for SlotChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        SlotChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for SlotChild {
    fn from(child: BidirectionalOverride) -> Self {
        SlotChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for SlotChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        SlotChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for SlotChild {
    fn from(child: BlockQuote) -> Self {
        SlotChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for SlotChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        SlotChild::BlockQuote(builder.build())
    }
}
impl From<Body> for SlotChild {
    fn from(child: Body) -> Self {
        SlotChild::Body(child)
    }
}
impl From<builders::BodyBuilder> for SlotChild {
    fn from(builder: builders::BodyBuilder) -> Self {
        SlotChild::Body(builder.build())
    }
}
impl From<Bold> for SlotChild {
    fn from(child: Bold) -> Self {
        SlotChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for SlotChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        SlotChild::Bold(builder.build())
    }
}
impl From<Button> for SlotChild {
    fn from(child: Button) -> Self {
        SlotChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for SlotChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        SlotChild::Button(builder.build())
    }
}
impl From<Canvas> for SlotChild {
    fn from(child: Canvas) -> Self {
        SlotChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for SlotChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        SlotChild::Canvas(builder.build())
    }
}
impl From<Caption> for SlotChild {
    fn from(child: Caption) -> Self {
        SlotChild::Caption(child)
    }
}
impl From<builders::CaptionBuilder> for SlotChild {
    fn from(builder: builders::CaptionBuilder) -> Self {
        SlotChild::Caption(builder.build())
    }
}
impl From<Cite> for SlotChild {
    fn from(child: Cite) -> Self {
        SlotChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for SlotChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        SlotChild::Cite(builder.build())
    }
}
impl From<Code> for SlotChild {
    fn from(child: Code) -> Self {
        SlotChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for SlotChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        SlotChild::Code(builder.build())
    }
}
impl From<Custom> for SlotChild {
    fn from(child: Custom) -> Self {
        SlotChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for SlotChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        SlotChild::Custom(builder.build())
    }
}
impl From<Data> for SlotChild {
    fn from(child: Data) -> Self {
        SlotChild::Data(child)
    }
}
impl From<builders::DataBuilder> for SlotChild {
    fn from(builder: builders::DataBuilder) -> Self {
        SlotChild::Data(builder.build())
    }
}
impl From<DataList> for SlotChild {
    fn from(child: DataList) -> Self {
        SlotChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for SlotChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        SlotChild::DataList(builder.build())
    }
}
impl From<Definition> for SlotChild {
    fn from(child: Definition) -> Self {
        SlotChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for SlotChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        SlotChild::Definition(builder.build())
    }
}
impl From<Deleted> for SlotChild {
    fn from(child: Deleted) -> Self {
        SlotChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for SlotChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        SlotChild::Deleted(builder.build())
    }
}
impl From<DescriptionDetails> for SlotChild {
    fn from(child: DescriptionDetails) -> Self {
        SlotChild::DescriptionDetails(child)
    }
}
impl From<builders::DescriptionDetailsBuilder> for SlotChild {
    fn from(builder: builders::DescriptionDetailsBuilder) -> Self {
        SlotChild::DescriptionDetails(builder.build())
    }
}
impl From<DescriptionList> for SlotChild {
    fn from(child: DescriptionList) -> Self {
        SlotChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for SlotChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        SlotChild::DescriptionList(builder.build())
    }
}
impl From<DescriptionTerm> for SlotChild {
    fn from(child: DescriptionTerm) -> Self {
        SlotChild::DescriptionTerm(child)
    }
}
impl From<builders::DescriptionTermBuilder> for SlotChild {
    fn from(builder: builders::DescriptionTermBuilder) -> Self {
        SlotChild::DescriptionTerm(builder.build())
    }
}
impl From<Details> for SlotChild {
    fn from(child: Details) -> Self {
        SlotChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for SlotChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        SlotChild::Details(builder.build())
    }
}
impl From<Dialog> for SlotChild {
    fn from(child: Dialog) -> Self {
        SlotChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for SlotChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        SlotChild::Dialog(builder.build())
    }
}
impl From<Division> for SlotChild {
    fn from(child: Division) -> Self {
        SlotChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for SlotChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        SlotChild::Division(builder.build())
    }
}
impl From<Embed> for SlotChild {
    fn from(child: Embed) -> Self {
        SlotChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for SlotChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        SlotChild::Embed(builder.build())
    }
}
impl From<Emphasis> for SlotChild {
    fn from(child: Emphasis) -> Self {
        SlotChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for SlotChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        SlotChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for SlotChild {
    fn from(child: FieldSet) -> Self {
        SlotChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for SlotChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        SlotChild::FieldSet(builder.build())
    }
}
impl From<FieldSetLegend> for SlotChild {
    fn from(child: FieldSetLegend) -> Self {
        SlotChild::FieldSetLegend(child)
    }
}
impl From<builders::FieldSetLegendBuilder> for SlotChild {
    fn from(builder: builders::FieldSetLegendBuilder) -> Self {
        SlotChild::FieldSetLegend(builder.build())
    }
}
impl From<Figure> for SlotChild {
    fn from(child: Figure) -> Self {
        SlotChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for SlotChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        SlotChild::Figure(builder.build())
    }
}
impl From<FigureCaption> for SlotChild {
    fn from(child: FigureCaption) -> Self {
        SlotChild::FigureCaption(child)
    }
}
impl From<builders::FigureCaptionBuilder> for SlotChild {
    fn from(builder: builders::FigureCaptionBuilder) -> Self {
        SlotChild::FigureCaption(builder.build())
    }
}
impl From<Footer> for SlotChild {
    fn from(child: Footer) -> Self {
        SlotChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for SlotChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        SlotChild::Footer(builder.build())
    }
}
impl From<Form> for SlotChild {
    fn from(child: Form) -> Self {
        SlotChild::Form(child)
    }
}
impl From<builders::FormBuilder> for SlotChild {
    fn from(builder: builders::FormBuilder) -> Self {
        SlotChild::Form(builder.build())
    }
}
impl From<Head> for SlotChild {
    fn from(child: Head) -> Self {
        SlotChild::Head(child)
    }
}
impl From<builders::HeadBuilder> for SlotChild {
    fn from(builder: builders::HeadBuilder) -> Self {
        SlotChild::Head(builder.build())
    }
}
impl From<Header> for SlotChild {
    fn from(child: Header) -> Self {
        SlotChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for SlotChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        SlotChild::Header(builder.build())
    }
}
impl From<Heading1> for SlotChild {
    fn from(child: Heading1) -> Self {
        SlotChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for SlotChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        SlotChild::Heading1(builder.build())
    }
}
impl From<Heading2> for SlotChild {
    fn from(child: Heading2) -> Self {
        SlotChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for SlotChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        SlotChild::Heading2(builder.build())
    }
}
impl From<Heading3> for SlotChild {
    fn from(child: Heading3) -> Self {
        SlotChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for SlotChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        SlotChild::Heading3(builder.build())
    }
}
impl From<Heading4> for SlotChild {
    fn from(child: Heading4) -> Self {
        SlotChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for SlotChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        SlotChild::Heading4(builder.build())
    }
}
impl From<Heading5> for SlotChild {
    fn from(child: Heading5) -> Self {
        SlotChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for SlotChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        SlotChild::Heading5(builder.build())
    }
}
impl From<Heading6> for SlotChild {
    fn from(child: Heading6) -> Self {
        SlotChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for SlotChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        SlotChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for SlotChild {
    fn from(child: HeadingGroup) -> Self {
        SlotChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for SlotChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        SlotChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for SlotChild {
    fn from(child: HorizontalRule) -> Self {
        SlotChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for SlotChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        SlotChild::HorizontalRule(builder.build())
    }
}
impl From<Html> for SlotChild {
    fn from(child: Html) -> Self {
        SlotChild::Html(child)
    }
}
impl From<builders::HtmlBuilder> for SlotChild {
    fn from(builder: builders::HtmlBuilder) -> Self {
        SlotChild::Html(builder.build())
    }
}
impl From<Image> for SlotChild {
    fn from(child: Image) -> Self {
        SlotChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for SlotChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        SlotChild::Image(builder.build())
    }
}
impl From<InlineFrame> for SlotChild {
    fn from(child: InlineFrame) -> Self {
        SlotChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for SlotChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        SlotChild::InlineFrame(builder.build())
    }
}
impl From<Input> for SlotChild {
    fn from(child: Input) -> Self {
        SlotChild::Input(child)
    }
}
impl From<builders::InputBuilder> for SlotChild {
    fn from(builder: builders::InputBuilder) -> Self {
        SlotChild::Input(builder.build())
    }
}
impl From<Inserted> for SlotChild {
    fn from(child: Inserted) -> Self {
        SlotChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for SlotChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        SlotChild::Inserted(builder.build())
    }
}
impl From<Italic> for SlotChild {
    fn from(child: Italic) -> Self {
        SlotChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for SlotChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        SlotChild::Italic(builder.build())
    }
}
impl From<Keyboard> for SlotChild {
    fn from(child: Keyboard) -> Self {
        SlotChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for SlotChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        SlotChild::Keyboard(builder.build())
    }
}
impl From<Label> for SlotChild {
    fn from(child: Label) -> Self {
        SlotChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for SlotChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        SlotChild::Label(builder.build())
    }
}
impl From<LineBreak> for SlotChild {
    fn from(child: LineBreak) -> Self {
        SlotChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for SlotChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        SlotChild::LineBreak(builder.build())
    }
}
impl From<Link> for SlotChild {
    fn from(child: Link) -> Self {
        SlotChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for SlotChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        SlotChild::Link(builder.build())
    }
}
impl From<ListItem> for SlotChild {
    fn from(child: ListItem) -> Self {
        SlotChild::ListItem(child)
    }
}
impl From<builders::ListItemBuilder> for SlotChild {
    fn from(builder: builders::ListItemBuilder) -> Self {
        SlotChild::ListItem(builder.build())
    }
}
impl From<Main> for SlotChild {
    fn from(child: Main) -> Self {
        SlotChild::Main(child)
    }
}
impl From<builders::MainBuilder> for SlotChild {
    fn from(builder: builders::MainBuilder) -> Self {
        SlotChild::Main(builder.build())
    }
}
impl From<Map> for SlotChild {
    fn from(child: Map) -> Self {
        SlotChild::Map(child)
    }
}
impl From<builders::MapBuilder> for SlotChild {
    fn from(builder: builders::MapBuilder) -> Self {
        SlotChild::Map(builder.build())
    }
}
impl From<MapArea> for SlotChild {
    fn from(child: MapArea) -> Self {
        SlotChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for SlotChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        SlotChild::MapArea(builder.build())
    }
}
impl From<Mark> for SlotChild {
    fn from(child: Mark) -> Self {
        SlotChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for SlotChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        SlotChild::Mark(builder.build())
    }
}
impl From<Menu> for SlotChild {
    fn from(child: Menu) -> Self {
        SlotChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for SlotChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        SlotChild::Menu(builder.build())
    }
}
impl From<Metadata> for SlotChild {
    fn from(child: Metadata) -> Self {
        SlotChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for SlotChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        SlotChild::Metadata(builder.build())
    }
}
impl From<Meter> for SlotChild {
    fn from(child: Meter) -> Self {
        SlotChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for SlotChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        SlotChild::Meter(builder.build())
    }
}
impl From<Navigation> for SlotChild {
    fn from(child: Navigation) -> Self {
        SlotChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for SlotChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        SlotChild::Navigation(builder.build())
    }
}
impl From<NoScript> for SlotChild {
    fn from(child: NoScript) -> Self {
        SlotChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for SlotChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        SlotChild::NoScript(builder.build())
    }
}
impl From<Object> for SlotChild {
    fn from(child: Object) -> Self {
        SlotChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for SlotChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        SlotChild::Object(builder.build())
    }
}
impl From<Option> for SlotChild {
    fn from(child: Option) -> Self {
        SlotChild::Option(child)
    }
}
impl From<builders::OptionBuilder> for SlotChild {
    fn from(builder: builders::OptionBuilder) -> Self {
        SlotChild::Option(builder.build())
    }
}
impl From<OptionGroup> for SlotChild {
    fn from(child: OptionGroup) -> Self {
        SlotChild::OptionGroup(child)
    }
}
impl From<builders::OptionGroupBuilder> for SlotChild {
    fn from(builder: builders::OptionGroupBuilder) -> Self {
        SlotChild::OptionGroup(builder.build())
    }
}
impl From<OrderedList> for SlotChild {
    fn from(child: OrderedList) -> Self {
        SlotChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for SlotChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        SlotChild::OrderedList(builder.build())
    }
}
impl From<Output> for SlotChild {
    fn from(child: Output) -> Self {
        SlotChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for SlotChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        SlotChild::Output(builder.build())
    }
}
impl From<Paragraph> for SlotChild {
    fn from(child: Paragraph) -> Self {
        SlotChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for SlotChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        SlotChild::Paragraph(builder.build())
    }
}
impl From<Picture> for SlotChild {
    fn from(child: Picture) -> Self {
        SlotChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for SlotChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        SlotChild::Picture(builder.build())
    }
}
impl From<Preformatted> for SlotChild {
    fn from(child: Preformatted) -> Self {
        SlotChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for SlotChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        SlotChild::Preformatted(builder.build())
    }
}
impl From<Progress> for SlotChild {
    fn from(child: Progress) -> Self {
        SlotChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for SlotChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        SlotChild::Progress(builder.build())
    }
}
impl From<Quote> for SlotChild {
    fn from(child: Quote) -> Self {
        SlotChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for SlotChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        SlotChild::Quote(builder.build())
    }
}
impl From<Ruby> for SlotChild {
    fn from(child: Ruby) -> Self {
        SlotChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for SlotChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        SlotChild::Ruby(builder.build())
    }
}
impl From<RubyParenthesis> for SlotChild {
    fn from(child: RubyParenthesis) -> Self {
        SlotChild::RubyParenthesis(child)
    }
}
impl From<builders::RubyParenthesisBuilder> for SlotChild {
    fn from(builder: builders::RubyParenthesisBuilder) -> Self {
        SlotChild::RubyParenthesis(builder.build())
    }
}
impl From<RubyText> for SlotChild {
    fn from(child: RubyText) -> Self {
        SlotChild::RubyText(child)
    }
}
impl From<builders::RubyTextBuilder> for SlotChild {
    fn from(builder: builders::RubyTextBuilder) -> Self {
        SlotChild::RubyText(builder.build())
    }
}
impl From<Sample> for SlotChild {
    fn from(child: Sample) -> Self {
        SlotChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for SlotChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        SlotChild::Sample(builder.build())
    }
}
impl From<Script> for SlotChild {
    fn from(child: Script) -> Self {
        SlotChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for SlotChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        SlotChild::Script(builder.build())
    }
}
impl From<Search> for SlotChild {
    fn from(child: Search) -> Self {
        SlotChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for SlotChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        SlotChild::Search(builder.build())
    }
}
impl From<Section> for SlotChild {
    fn from(child: Section) -> Self {
        SlotChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for SlotChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        SlotChild::Section(builder.build())
    }
}
impl From<Select> for SlotChild {
    fn from(child: Select) -> Self {
        SlotChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for SlotChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        SlotChild::Select(builder.build())
    }
}
impl From<Slot> for SlotChild {
    fn from(child: Slot) -> Self {
        SlotChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for SlotChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        SlotChild::Slot(builder.build())
    }
}
impl From<Small> for SlotChild {
    fn from(child: Small) -> Self {
        SlotChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for SlotChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        SlotChild::Small(builder.build())
    }
}
impl From<Source> for SlotChild {
    fn from(child: Source) -> Self {
        SlotChild::Source(child)
    }
}
impl From<builders::SourceBuilder> for SlotChild {
    fn from(builder: builders::SourceBuilder) -> Self {
        SlotChild::Source(builder.build())
    }
}
impl From<Span> for SlotChild {
    fn from(child: Span) -> Self {
        SlotChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for SlotChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        SlotChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for SlotChild {
    fn from(child: StrikeThrough) -> Self {
        SlotChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for SlotChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        SlotChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for SlotChild {
    fn from(child: Strong) -> Self {
        SlotChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for SlotChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        SlotChild::Strong(builder.build())
    }
}
impl From<Style> for SlotChild {
    fn from(child: Style) -> Self {
        SlotChild::Style(child)
    }
}
impl From<builders::StyleBuilder> for SlotChild {
    fn from(builder: builders::StyleBuilder) -> Self {
        SlotChild::Style(builder.build())
    }
}
impl From<SubScript> for SlotChild {
    fn from(child: SubScript) -> Self {
        SlotChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for SlotChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        SlotChild::SubScript(builder.build())
    }
}
impl From<Summary> for SlotChild {
    fn from(child: Summary) -> Self {
        SlotChild::Summary(child)
    }
}
impl From<builders::SummaryBuilder> for SlotChild {
    fn from(builder: builders::SummaryBuilder) -> Self {
        SlotChild::Summary(builder.build())
    }
}
impl From<SupScript> for SlotChild {
    fn from(child: SupScript) -> Self {
        SlotChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for SlotChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        SlotChild::SupScript(builder.build())
    }
}
impl From<Table> for SlotChild {
    fn from(child: Table) -> Self {
        SlotChild::Table(child)
    }
}
impl From<builders::TableBuilder> for SlotChild {
    fn from(builder: builders::TableBuilder) -> Self {
        SlotChild::Table(builder.build())
    }
}
impl From<TableBody> for SlotChild {
    fn from(child: TableBody) -> Self {
        SlotChild::TableBody(child)
    }
}
impl From<builders::TableBodyBuilder> for SlotChild {
    fn from(builder: builders::TableBodyBuilder) -> Self {
        SlotChild::TableBody(builder.build())
    }
}
impl From<TableCell> for SlotChild {
    fn from(child: TableCell) -> Self {
        SlotChild::TableCell(child)
    }
}
impl From<builders::TableCellBuilder> for SlotChild {
    fn from(builder: builders::TableCellBuilder) -> Self {
        SlotChild::TableCell(builder.build())
    }
}
impl From<TableColumn> for SlotChild {
    fn from(child: TableColumn) -> Self {
        SlotChild::TableColumn(child)
    }
}
impl From<builders::TableColumnBuilder> for SlotChild {
    fn from(builder: builders::TableColumnBuilder) -> Self {
        SlotChild::TableColumn(builder.build())
    }
}
impl From<TableColumnGroup> for SlotChild {
    fn from(child: TableColumnGroup) -> Self {
        SlotChild::TableColumnGroup(child)
    }
}
impl From<builders::TableColumnGroupBuilder> for SlotChild {
    fn from(builder: builders::TableColumnGroupBuilder) -> Self {
        SlotChild::TableColumnGroup(builder.build())
    }
}
impl From<TableFoot> for SlotChild {
    fn from(child: TableFoot) -> Self {
        SlotChild::TableFoot(child)
    }
}
impl From<builders::TableFootBuilder> for SlotChild {
    fn from(builder: builders::TableFootBuilder) -> Self {
        SlotChild::TableFoot(builder.build())
    }
}
impl From<TableHead> for SlotChild {
    fn from(child: TableHead) -> Self {
        SlotChild::TableHead(child)
    }
}
impl From<builders::TableHeadBuilder> for SlotChild {
    fn from(builder: builders::TableHeadBuilder) -> Self {
        SlotChild::TableHead(builder.build())
    }
}
impl From<TableHeader> for SlotChild {
    fn from(child: TableHeader) -> Self {
        SlotChild::TableHeader(child)
    }
}
impl From<builders::TableHeaderBuilder> for SlotChild {
    fn from(builder: builders::TableHeaderBuilder) -> Self {
        SlotChild::TableHeader(builder.build())
    }
}
impl From<TableRow> for SlotChild {
    fn from(child: TableRow) -> Self {
        SlotChild::TableRow(child)
    }
}
impl From<builders::TableRowBuilder> for SlotChild {
    fn from(builder: builders::TableRowBuilder) -> Self {
        SlotChild::TableRow(builder.build())
    }
}
impl From<Template> for SlotChild {
    fn from(child: Template) -> Self {
        SlotChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for SlotChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        SlotChild::Template(builder.build())
    }
}
impl From<TextArea> for SlotChild {
    fn from(child: TextArea) -> Self {
        SlotChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for SlotChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        SlotChild::TextArea(builder.build())
    }
}
impl From<Time> for SlotChild {
    fn from(child: Time) -> Self {
        SlotChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for SlotChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        SlotChild::Time(builder.build())
    }
}
impl From<Title> for SlotChild {
    fn from(child: Title) -> Self {
        SlotChild::Title(child)
    }
}
impl From<builders::TitleBuilder> for SlotChild {
    fn from(builder: builders::TitleBuilder) -> Self {
        SlotChild::Title(builder.build())
    }
}
impl From<Track> for SlotChild {
    fn from(child: Track) -> Self {
        SlotChild::Track(child)
    }
}
impl From<builders::TrackBuilder> for SlotChild {
    fn from(builder: builders::TrackBuilder) -> Self {
        SlotChild::Track(builder.build())
    }
}
impl From<Underline> for SlotChild {
    fn from(child: Underline) -> Self {
        SlotChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for SlotChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        SlotChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for SlotChild {
    fn from(child: UnorderedList) -> Self {
        SlotChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for SlotChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        SlotChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for SlotChild {
    fn from(child: Variable) -> Self {
        SlotChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for SlotChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        SlotChild::Variable(builder.build())
    }
}
impl From<Video> for SlotChild {
    fn from(child: Video) -> Self {
        SlotChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for SlotChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        SlotChild::Video(builder.build())
    }
}
impl From<WordBreak> for SlotChild {
    fn from(child: WordBreak) -> Self {
        SlotChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for SlotChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        SlotChild::WordBreak(builder.build())
    }
}
impl std::fmt::Debug for SlotChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SlotChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Address(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Article(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Base(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Body(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Button(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Caption(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Code(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Data(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::DescriptionDetails(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::DescriptionTerm(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Details(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Division(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::FieldSetLegend(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::FigureCaption(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Form(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Head(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Header(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Html(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Image(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Input(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Label(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Link(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::ListItem(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Main(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Map(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Object(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Option(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::OptionGroup(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Output(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::RubyParenthesis(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::RubyText(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Script(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Search(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Section(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Select(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Small(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Source(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Span(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Style(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Summary(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Table(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::TableBody(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::TableCell(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::TableColumn(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::TableColumnGroup(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::TableFoot(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::TableHead(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::TableHeader(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::TableRow(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Template(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Time(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Title(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Track(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::Video(child) => std::fmt::Debug::fmt(child, f),
            SlotChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for SlotChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SlotChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Address(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Article(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Aside(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Audio(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Base(child) => std::fmt::Display::fmt(child, f),
            SlotChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            SlotChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            SlotChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Body(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Bold(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Button(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Caption(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Cite(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Code(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Custom(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Data(child) => std::fmt::Display::fmt(child, f),
            SlotChild::DataList(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Definition(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            SlotChild::DescriptionDetails(child) => std::fmt::Display::fmt(child, f),
            SlotChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            SlotChild::DescriptionTerm(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Details(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Division(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Embed(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            SlotChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            SlotChild::FieldSetLegend(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Figure(child) => std::fmt::Display::fmt(child, f),
            SlotChild::FigureCaption(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Footer(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Form(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Head(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Header(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            SlotChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            SlotChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Html(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Image(child) => std::fmt::Display::fmt(child, f),
            SlotChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Input(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Italic(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Label(child) => std::fmt::Display::fmt(child, f),
            SlotChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Link(child) => std::fmt::Display::fmt(child, f),
            SlotChild::ListItem(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Main(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Map(child) => std::fmt::Display::fmt(child, f),
            SlotChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Mark(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Menu(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Meter(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            SlotChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Object(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Option(child) => std::fmt::Display::fmt(child, f),
            SlotChild::OptionGroup(child) => std::fmt::Display::fmt(child, f),
            SlotChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Output(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Picture(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Progress(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Quote(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            SlotChild::RubyParenthesis(child) => std::fmt::Display::fmt(child, f),
            SlotChild::RubyText(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Sample(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Script(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Search(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Section(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Select(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Slot(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Small(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Source(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Span(child) => std::fmt::Display::fmt(child, f),
            SlotChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Strong(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Style(child) => std::fmt::Display::fmt(child, f),
            SlotChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Summary(child) => std::fmt::Display::fmt(child, f),
            SlotChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Table(child) => std::fmt::Display::fmt(child, f),
            SlotChild::TableBody(child) => std::fmt::Display::fmt(child, f),
            SlotChild::TableCell(child) => std::fmt::Display::fmt(child, f),
            SlotChild::TableColumn(child) => std::fmt::Display::fmt(child, f),
            SlotChild::TableColumnGroup(child) => std::fmt::Display::fmt(child, f),
            SlotChild::TableFoot(child) => std::fmt::Display::fmt(child, f),
            SlotChild::TableHead(child) => std::fmt::Display::fmt(child, f),
            SlotChild::TableHeader(child) => std::fmt::Display::fmt(child, f),
            SlotChild::TableRow(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Template(child) => std::fmt::Display::fmt(child, f),
            SlotChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Time(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Title(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Track(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Underline(child) => std::fmt::Display::fmt(child, f),
            SlotChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Variable(child) => std::fmt::Display::fmt(child, f),
            SlotChild::Video(child) => std::fmt::Display::fmt(child, f),
            SlotChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
