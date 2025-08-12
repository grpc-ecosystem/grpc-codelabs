const _: () = ::protobuf::__internal::assert_compatible_gencode_version(
    "4.31.1-release",
);
#[allow(non_camel_case_types)]
pub struct Point {
    inner: ::protobuf::__internal::runtime::MessageInner,
}
impl ::protobuf::Message for Point {}
impl ::std::default::Default for Point {
    fn default() -> Self {
        Self::new()
    }
}
impl ::protobuf::Parse for Point {
    fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        Self::parse(serialized)
    }
}
impl ::std::fmt::Debug for Point {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::TakeFrom for Point {
    fn take_from(&mut self, src: impl ::protobuf::AsMut<MutProxied = Self>) {
        let mut m = self.as_mut();
        ::protobuf::TakeFrom::take_from(&mut m, src)
    }
}
impl ::protobuf::CopyFrom for Point {
    fn copy_from(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
        let mut m = self.as_mut();
        ::protobuf::CopyFrom::copy_from(&mut m, src)
    }
}
impl ::protobuf::MergeFrom for Point {
    fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
        let mut m = self.as_mut();
        ::protobuf::MergeFrom::merge_from(&mut m, src)
    }
}
impl ::protobuf::Serialize for Point {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        ::protobuf::AsView::as_view(self).serialize()
    }
}
impl ::protobuf::Clear for Point {
    fn clear(&mut self) {
        let mut m = self.as_mut();
        ::protobuf::Clear::clear(&mut m)
    }
}
impl ::protobuf::ClearAndParse for Point {
    fn clear_and_parse(
        &mut self,
        data: &[u8],
    ) -> ::std::result::Result<(), ::protobuf::ParseError> {
        let mut m = self.as_mut();
        ::protobuf::ClearAndParse::clear_and_parse(&mut m, data)
    }
}
unsafe impl Sync for Point {}
unsafe impl Send for Point {}
impl ::protobuf::Proxied for Point {
    type View<'msg> = PointView<'msg>;
}
impl ::protobuf::__internal::SealedInternal for Point {}
impl ::protobuf::MutProxied for Point {
    type Mut<'msg> = PointMut<'msg>;
}
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PointView<'msg> {
    msg: ::protobuf::__internal::runtime::RawMessage,
    _phantom: ::std::marker::PhantomData<&'msg ()>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for PointView<'msg> {}
impl<'msg> ::protobuf::MessageView<'msg> for PointView<'msg> {
    type Message = Point;
}
impl ::std::fmt::Debug for PointView<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for PointView<'_> {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        let encoded = unsafe {
            ::protobuf::__internal::runtime::wire::encode(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        encoded.map_err(|_| ::protobuf::SerializeError)
    }
}
impl ::std::default::Default for PointView<'_> {
    fn default() -> PointView<'static> {
        PointView::new(
            ::protobuf::__internal::Private,
            ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
        )
    }
}
#[allow(dead_code)]
impl<'msg> PointView<'msg> {
    #[doc(hidden)]
    pub fn new(
        _private: ::protobuf::__internal::Private,
        msg: ::protobuf::__internal::runtime::RawMessage,
    ) -> Self {
        Self {
            msg,
            _phantom: ::std::marker::PhantomData,
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.msg
    }
    pub fn to_owned(&self) -> Point {
        ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
    }
    pub fn latitude(self) -> i32 {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_GetInt32(
                    self.raw_msg(),
                    f,
                    (0i32).into(),
                )
                .try_into()
                .unwrap()
        }
    }
    pub fn longitude(self) -> i32 {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_GetInt32(
                    self.raw_msg(),
                    f,
                    (0i32).into(),
                )
                .try_into()
                .unwrap()
        }
    }
}
unsafe impl Sync for PointView<'_> {}
unsafe impl Send for PointView<'_> {}
impl<'msg> ::protobuf::Proxy<'msg> for PointView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PointView<'msg> {}
impl<'msg> ::protobuf::AsView for PointView<'msg> {
    type Proxied = Point;
    fn as_view(&self) -> ::protobuf::View<'msg, Point> {
        *self
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for PointView<'msg> {
    fn into_view<'shorter>(self) -> PointView<'shorter>
    where
        'msg: 'shorter,
    {
        self
    }
}
impl<'msg> ::protobuf::IntoProxied<Point> for PointView<'msg> {
    fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Point {
        let dst = Point::new();
        unsafe {
            ::protobuf::__internal::runtime::upb_Message_DeepCopy(
                dst.inner.msg,
                self.msg,
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                dst.inner.arena.raw(),
            )
        };
        dst
    }
}
impl<'msg> ::protobuf::IntoProxied<Point> for PointMut<'msg> {
    fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Point {
        ::protobuf::IntoProxied::into_proxied(
            ::protobuf::IntoView::into_view(self),
            _private,
        )
    }
}
unsafe impl ::protobuf::ProxiedInRepeated for Point {
    fn repeated_new(
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::Repeated<Self> {
        let arena = ::protobuf::__internal::runtime::Arena::new();
        unsafe {
            ::protobuf::Repeated::from_inner(
                ::protobuf::__internal::Private,
                ::protobuf::__internal::runtime::InnerRepeated::from_raw_parts(
                    ::protobuf::__internal::runtime::upb_Array_New(
                        arena.raw(),
                        ::protobuf::__internal::runtime::CType::Message,
                    ),
                    arena,
                ),
            )
        }
    }
    unsafe fn repeated_free(
        _private: ::protobuf::__internal::Private,
        _f: &mut ::protobuf::Repeated<Self>,
    ) {}
    fn repeated_len(f: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
        unsafe {
            ::protobuf::__internal::runtime::upb_Array_Size(
                f.as_raw(::protobuf::__internal::Private),
            )
        }
    }
    unsafe fn repeated_set_unchecked(
        mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
        i: usize,
        v: impl ::protobuf::IntoProxied<Self>,
    ) {
        unsafe {
            ::protobuf::__internal::runtime::upb_Array_Set(
                f.as_raw(::protobuf::__internal::Private),
                i,
                <Self as ::protobuf::__internal::runtime::UpbTypeConversions>::into_message_value_fuse_if_required(
                    f.raw_arena(::protobuf::__internal::Private),
                    v.into_proxied(::protobuf::__internal::Private),
                ),
            )
        }
    }
    unsafe fn repeated_get_unchecked(
        f: ::protobuf::View<::protobuf::Repeated<Self>>,
        i: usize,
    ) -> ::protobuf::View<Self> {
        let msg_ptr = unsafe {
            ::protobuf::__internal::runtime::upb_Array_Get(
                    f.as_raw(::protobuf::__internal::Private),
                    i,
                )
                .msg_val
        }
            .expect("upb_Array* element should not be NULL.");
        ::protobuf::View::<Self>::new(::protobuf::__internal::Private, msg_ptr)
    }
    unsafe fn repeated_get_mut_unchecked(
        mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
        i: usize,
    ) -> ::protobuf::Mut<Self> {
        let msg_ptr = unsafe {
            ::protobuf::__internal::runtime::upb_Array_GetMutable(
                f.as_raw(::protobuf::__internal::Private),
                i,
            )
        };
        unsafe {
            ::protobuf::Mut::<Self> {
                inner: ::protobuf::__internal::runtime::MutatorMessageRef::from_raw_parts(
                    msg_ptr,
                    f.arena(::protobuf::__internal::Private),
                ),
            }
        }
    }
    fn repeated_clear(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
        unsafe {
            ::protobuf::__internal::runtime::upb_Array_Resize(
                f.as_raw(::protobuf::__internal::Private),
                0,
                f.raw_arena(::protobuf::__internal::Private),
            )
        };
    }
    fn repeated_push(
        mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
        v: impl ::protobuf::IntoProxied<Self>,
    ) {
        unsafe {
            ::protobuf::__internal::runtime::upb_Array_Append(
                f.as_raw(::protobuf::__internal::Private),
                <Self as ::protobuf::__internal::runtime::UpbTypeConversions>::into_message_value_fuse_if_required(
                    f.raw_arena(::protobuf::__internal::Private),
                    v.into_proxied(::protobuf::__internal::Private),
                ),
                f.raw_arena(::protobuf::__internal::Private),
            );
        };
    }
    fn repeated_copy_from(
        src: ::protobuf::View<::protobuf::Repeated<Self>>,
        dest: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    ) {
        unsafe {
            ::protobuf::__internal::runtime::repeated_message_copy_from(
                src,
                dest,
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            );
        }
    }
    fn repeated_reserve(
        mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
        additional: usize,
    ) {
        unsafe {
            let size = ::protobuf::__internal::runtime::upb_Array_Size(
                f.as_raw(::protobuf::__internal::Private),
            );
            ::protobuf::__internal::runtime::upb_Array_Reserve(
                f.as_raw(::protobuf::__internal::Private),
                size + additional,
                f.raw_arena(::protobuf::__internal::Private),
            );
        }
    }
}
impl ::protobuf::__internal::runtime::UpbTypeConversions for Point {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }
    fn to_message_value(
        val: ::protobuf::View<'_, Self>,
    ) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue {
            msg_val: Some(val.raw_msg()),
        }
    }
    unsafe fn into_message_value_fuse_if_required(
        raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
        mut val: Self,
    ) -> ::protobuf::__internal::runtime::upb_MessageValue {
        let parent_arena = ::std::mem::ManuallyDrop::new(unsafe {
            ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena)
        });
        parent_arena
            .fuse(val.as_mutator_message_ref(::protobuf::__internal::Private).arena());
        ::protobuf::__internal::runtime::upb_MessageValue {
            msg_val: Some(val.raw_msg()),
        }
    }
    unsafe fn from_message_value<'msg>(
        msg: ::protobuf::__internal::runtime::upb_MessageValue,
    ) -> ::protobuf::View<'msg, Self> {
        PointView::new(
            ::protobuf::__internal::Private,
            unsafe { msg.msg_val }.expect("expected present message value in map"),
        )
    }
    unsafe fn from_message_mut<'msg>(
        msg: *mut ::protobuf::__internal::runtime::upb_Message,
        arena: &'msg ::protobuf::__internal::runtime::Arena,
    ) -> PointMut<'msg> {
        PointMut {
            inner: unsafe {
                ::protobuf::__internal::runtime::MutatorMessageRef::from_raw_parts(
                    std::ptr::NonNull::new(msg)
                        .expect("expected present message value in map"),
                    arena,
                )
            },
        }
    }
}
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PointMut<'msg> {
    inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for PointMut<'msg> {}
impl<'msg> ::protobuf::MessageMut<'msg> for PointMut<'msg> {
    type Message = Point;
}
impl ::std::fmt::Debug for PointMut<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for PointMut<'_> {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        ::protobuf::AsView::as_view(self).serialize()
    }
}
impl ::protobuf::Clear for PointMut<'_> {
    fn clear(&mut self) {
        unsafe {
            ::protobuf::__internal::runtime::upb_Message_Clear(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        }
    }
}
impl ::protobuf::ClearAndParse for PointMut<'_> {
    fn clear_and_parse(
        &mut self,
        data: &[u8],
    ) -> ::std::result::Result<(), ::protobuf::ParseError> {
        ::protobuf::Clear::clear(self);
        let status = unsafe {
            ::protobuf::__internal::runtime::wire::decode(
                data,
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                self.arena(),
            )
        };
        match status {
            Ok(_) => Ok(()),
            Err(_) => Err(::protobuf::ParseError),
        }
    }
}
impl ::protobuf::TakeFrom for PointMut<'_> {
    fn take_from(&mut self, mut src: impl ::protobuf::AsMut<MutProxied = Point>) {
        let mut src = src.as_mut();
        ::protobuf::CopyFrom::copy_from(self, ::protobuf::AsView::as_view(&src));
        ::protobuf::Clear::clear(&mut src);
    }
}
impl ::protobuf::CopyFrom for PointMut<'_> {
    fn copy_from(&mut self, src: impl ::protobuf::AsView<Proxied = Point>) {
        unsafe {
            assert!(
                ::protobuf::__internal::runtime::upb_Message_DeepCopy(self.raw_msg(), src
                .as_view().raw_msg(), < Self as
                ::protobuf::__internal::runtime::AssociatedMiniTable >::mini_table(),
                self.arena().raw())
            );
        }
    }
}
impl ::protobuf::MergeFrom for PointMut<'_> {
    fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = Point>) {
        unsafe {
            assert!(
                ::protobuf::__internal::runtime::upb_Message_MergeFrom(self.raw_msg(),
                src.as_view().raw_msg(), < Self as
                ::protobuf::__internal::runtime::AssociatedMiniTable >::mini_table(),
                ::std::ptr::null(), self.arena().raw())
            );
        }
    }
}
#[allow(dead_code)]
impl<'msg> PointMut<'msg> {
    #[doc(hidden)]
    pub fn from_parent(
        _private: ::protobuf::__internal::Private,
        parent: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
        msg: ::protobuf::__internal::runtime::RawMessage,
    ) -> Self {
        Self {
            inner: ::protobuf::__internal::runtime::MutatorMessageRef::from_parent(
                parent,
                msg,
            ),
        }
    }
    #[doc(hidden)]
    pub fn new(
        _private: ::protobuf::__internal::Private,
        msg: &'msg mut ::protobuf::__internal::runtime::MessageInner,
    ) -> Self {
        Self {
            inner: ::protobuf::__internal::runtime::MutatorMessageRef::new(msg),
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.msg()
    }
    #[doc(hidden)]
    pub fn as_mutator_message_ref(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MutatorMessageRef<'msg> {
        self.inner
    }
    pub fn to_owned(&self) -> Point {
        ::protobuf::AsView::as_view(self).to_owned()
    }
    fn arena(&self) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
    pub fn latitude(&self) -> i32 {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_GetInt32(
                    self.raw_msg(),
                    f,
                    (0i32).into(),
                )
                .try_into()
                .unwrap()
        }
    }
    pub fn set_latitude(&mut self, val: i32) {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldInt32(
                self.raw_msg(),
                f,
                val.into(),
            );
        }
    }
    pub fn longitude(&self) -> i32 {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_GetInt32(
                    self.raw_msg(),
                    f,
                    (0i32).into(),
                )
                .try_into()
                .unwrap()
        }
    }
    pub fn set_longitude(&mut self, val: i32) {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldInt32(
                self.raw_msg(),
                f,
                val.into(),
            );
        }
    }
}
unsafe impl Sync for PointMut<'_> {}
impl<'msg> ::protobuf::Proxy<'msg> for PointMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PointMut<'msg> {}
impl<'msg> ::protobuf::AsView for PointMut<'msg> {
    type Proxied = Point;
    fn as_view(&self) -> ::protobuf::View<'_, Point> {
        PointView {
            msg: self.raw_msg(),
            _phantom: ::std::marker::PhantomData,
        }
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for PointMut<'msg> {
    fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Point>
    where
        'msg: 'shorter,
    {
        PointView {
            msg: self.raw_msg(),
            _phantom: ::std::marker::PhantomData,
        }
    }
}
impl<'msg> ::protobuf::AsMut for PointMut<'msg> {
    type MutProxied = Point;
    fn as_mut(&mut self) -> PointMut<'msg> {
        PointMut { inner: self.inner }
    }
}
impl<'msg> ::protobuf::IntoMut<'msg> for PointMut<'msg> {
    fn into_mut<'shorter>(self) -> PointMut<'shorter>
    where
        'msg: 'shorter,
    {
        self
    }
}
#[allow(dead_code)]
impl Point {
    pub fn new() -> Self {
        let arena = ::protobuf::__internal::runtime::Arena::new();
        let raw_msg = unsafe {
            ::protobuf::__internal::runtime::upb_Message_New(
                    <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                    arena.raw(),
                )
                .unwrap()
        };
        Self {
            inner: ::protobuf::__internal::runtime::MessageInner {
                msg: raw_msg,
                arena,
            },
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.msg
    }
    #[doc(hidden)]
    pub fn as_mutator_message_ref(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MutatorMessageRef {
        ::protobuf::__internal::runtime::MutatorMessageRef::new(&mut self.inner)
    }
    fn arena(&self) -> &::protobuf::__internal::runtime::Arena {
        &self.inner.arena
    }
    pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        let mut msg = Self::new();
        ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
    }
    pub fn as_view(&self) -> PointView {
        PointView::new(::protobuf::__internal::Private, self.inner.msg)
    }
    pub fn as_mut(&mut self) -> PointMut {
        PointMut::new(::protobuf::__internal::Private, &mut self.inner)
    }
    pub fn latitude(&self) -> i32 {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_GetInt32(
                    self.raw_msg(),
                    f,
                    (0i32).into(),
                )
                .try_into()
                .unwrap()
        }
    }
    pub fn set_latitude(&mut self, val: i32) {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldInt32(
                self.raw_msg(),
                f,
                val.into(),
            );
        }
    }
    pub fn longitude(&self) -> i32 {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_GetInt32(
                    self.raw_msg(),
                    f,
                    (0i32).into(),
                )
                .try_into()
                .unwrap()
        }
    }
    pub fn set_longitude(&mut self, val: i32) {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldInt32(
                self.raw_msg(),
                f,
                val.into(),
            );
        }
    }
}
impl ::std::ops::Drop for Point {
    fn drop(&mut self) {}
}
impl ::std::clone::Clone for Point {
    fn clone(&self) -> Self {
        self.as_view().to_owned()
    }
}
impl ::protobuf::AsView for Point {
    type Proxied = Self;
    fn as_view(&self) -> PointView {
        self.as_view()
    }
}
impl ::protobuf::AsMut for Point {
    type MutProxied = Self;
    fn as_mut(&mut self) -> PointMut {
        self.as_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Point {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        #[allow(unused_unsafe)]
        unsafe { ::std::ptr::addr_of!(routeguide__Point_msg_init) }
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PointView<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        #[allow(unused_unsafe)]
        unsafe { ::std::ptr::addr_of!(routeguide__Point_msg_init) }
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PointMut<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        #[allow(unused_unsafe)]
        unsafe { ::std::ptr::addr_of!(routeguide__Point_msg_init) }
    }
}
extern "C" {
    /// Opaque static extern for this message's MiniTable, generated
    /// by the upb C MiniTable codegen. The only valid way to
    /// reference this static is with `std::ptr::addr_of!(..)`.
    static routeguide__Point_msg_init: ::protobuf::__internal::runtime::upb_MiniTable;
}
impl ::protobuf::OwnedMessageInterop for Point {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PointMut<'a> {}
impl<'a> ::protobuf::MessageViewInterop<'a> for PointView<'a> {
    unsafe fn __unstable_wrap_raw_message(msg: &'a *const ::std::ffi::c_void) -> Self {
        Self::new(
            ::protobuf::__internal::Private,
            ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap(),
        )
    }
    unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
        msg: *const ::std::ffi::c_void,
    ) -> Self {
        Self::new(
            ::protobuf::__internal::Private,
            ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap(),
        )
    }
    fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
        self.msg.as_ptr() as *const _
    }
}
impl ::protobuf::__internal::MatcherEq for Point {
    fn matches(&self, o: &Self) -> bool {
        ::protobuf::__internal::MatcherEq::matches(
            &::protobuf::AsView::as_view(self),
            &::protobuf::AsView::as_view(o),
        )
    }
}
impl<'a> ::protobuf::__internal::MatcherEq for PointMut<'a> {
    fn matches(&self, o: &Self) -> bool {
        ::protobuf::__internal::MatcherEq::matches(
            &::protobuf::AsView::as_view(self),
            &::protobuf::AsView::as_view(o),
        )
    }
}
impl<'a> ::protobuf::__internal::MatcherEq for PointView<'a> {
    fn matches(&self, o: &Self) -> bool {
        unsafe {
            ::protobuf::__internal::runtime::upb_Message_IsEqual(
                self.msg,
                o.msg,
                <Point as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            )
        }
    }
}
#[allow(non_camel_case_types)]
pub struct Rectangle {
    inner: ::protobuf::__internal::runtime::MessageInner,
}
impl ::protobuf::Message for Rectangle {}
impl ::std::default::Default for Rectangle {
    fn default() -> Self {
        Self::new()
    }
}
impl ::protobuf::Parse for Rectangle {
    fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        Self::parse(serialized)
    }
}
impl ::std::fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::TakeFrom for Rectangle {
    fn take_from(&mut self, src: impl ::protobuf::AsMut<MutProxied = Self>) {
        let mut m = self.as_mut();
        ::protobuf::TakeFrom::take_from(&mut m, src)
    }
}
impl ::protobuf::CopyFrom for Rectangle {
    fn copy_from(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
        let mut m = self.as_mut();
        ::protobuf::CopyFrom::copy_from(&mut m, src)
    }
}
impl ::protobuf::MergeFrom for Rectangle {
    fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
        let mut m = self.as_mut();
        ::protobuf::MergeFrom::merge_from(&mut m, src)
    }
}
impl ::protobuf::Serialize for Rectangle {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        ::protobuf::AsView::as_view(self).serialize()
    }
}
impl ::protobuf::Clear for Rectangle {
    fn clear(&mut self) {
        let mut m = self.as_mut();
        ::protobuf::Clear::clear(&mut m)
    }
}
impl ::protobuf::ClearAndParse for Rectangle {
    fn clear_and_parse(
        &mut self,
        data: &[u8],
    ) -> ::std::result::Result<(), ::protobuf::ParseError> {
        let mut m = self.as_mut();
        ::protobuf::ClearAndParse::clear_and_parse(&mut m, data)
    }
}
unsafe impl Sync for Rectangle {}
unsafe impl Send for Rectangle {}
impl ::protobuf::Proxied for Rectangle {
    type View<'msg> = RectangleView<'msg>;
}
impl ::protobuf::__internal::SealedInternal for Rectangle {}
impl ::protobuf::MutProxied for Rectangle {
    type Mut<'msg> = RectangleMut<'msg>;
}
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RectangleView<'msg> {
    msg: ::protobuf::__internal::runtime::RawMessage,
    _phantom: ::std::marker::PhantomData<&'msg ()>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for RectangleView<'msg> {}
impl<'msg> ::protobuf::MessageView<'msg> for RectangleView<'msg> {
    type Message = Rectangle;
}
impl ::std::fmt::Debug for RectangleView<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for RectangleView<'_> {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        let encoded = unsafe {
            ::protobuf::__internal::runtime::wire::encode(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        encoded.map_err(|_| ::protobuf::SerializeError)
    }
}
impl ::std::default::Default for RectangleView<'_> {
    fn default() -> RectangleView<'static> {
        RectangleView::new(
            ::protobuf::__internal::Private,
            ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
        )
    }
}
#[allow(dead_code)]
impl<'msg> RectangleView<'msg> {
    #[doc(hidden)]
    pub fn new(
        _private: ::protobuf::__internal::Private,
        msg: ::protobuf::__internal::runtime::RawMessage,
    ) -> Self {
        Self {
            msg,
            _phantom: ::std::marker::PhantomData,
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.msg
    }
    pub fn to_owned(&self) -> Rectangle {
        ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
    }
    pub fn has_lo(self) -> bool {
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_HasBaseField(self.raw_msg(), f)
        }
    }
    pub fn lo_opt(self) -> ::protobuf::Optional<super::PointView<'msg>> {
        ::protobuf::Optional::new(self.lo(), self.has_lo())
    }
    pub fn lo(self) -> super::PointView<'msg> {
        let submsg = unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_GetMessage(self.raw_msg(), f)
        };
        match submsg {
            None => {
                super::PointView::new(
                    ::protobuf::__internal::Private,
                    ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
                )
            }
            Some(sub_raw_msg) => {
                super::PointView::new(::protobuf::__internal::Private, sub_raw_msg)
            }
        }
    }
    pub fn has_hi(self) -> bool {
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_HasBaseField(self.raw_msg(), f)
        }
    }
    pub fn hi_opt(self) -> ::protobuf::Optional<super::PointView<'msg>> {
        ::protobuf::Optional::new(self.hi(), self.has_hi())
    }
    pub fn hi(self) -> super::PointView<'msg> {
        let submsg = unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_GetMessage(self.raw_msg(), f)
        };
        match submsg {
            None => {
                super::PointView::new(
                    ::protobuf::__internal::Private,
                    ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
                )
            }
            Some(sub_raw_msg) => {
                super::PointView::new(::protobuf::__internal::Private, sub_raw_msg)
            }
        }
    }
}
unsafe impl Sync for RectangleView<'_> {}
unsafe impl Send for RectangleView<'_> {}
impl<'msg> ::protobuf::Proxy<'msg> for RectangleView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for RectangleView<'msg> {}
impl<'msg> ::protobuf::AsView for RectangleView<'msg> {
    type Proxied = Rectangle;
    fn as_view(&self) -> ::protobuf::View<'msg, Rectangle> {
        *self
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for RectangleView<'msg> {
    fn into_view<'shorter>(self) -> RectangleView<'shorter>
    where
        'msg: 'shorter,
    {
        self
    }
}
impl<'msg> ::protobuf::IntoProxied<Rectangle> for RectangleView<'msg> {
    fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Rectangle {
        let dst = Rectangle::new();
        unsafe {
            ::protobuf::__internal::runtime::upb_Message_DeepCopy(
                dst.inner.msg,
                self.msg,
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                dst.inner.arena.raw(),
            )
        };
        dst
    }
}
impl<'msg> ::protobuf::IntoProxied<Rectangle> for RectangleMut<'msg> {
    fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Rectangle {
        ::protobuf::IntoProxied::into_proxied(
            ::protobuf::IntoView::into_view(self),
            _private,
        )
    }
}
unsafe impl ::protobuf::ProxiedInRepeated for Rectangle {
    fn repeated_new(
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::Repeated<Self> {
        let arena = ::protobuf::__internal::runtime::Arena::new();
        unsafe {
            ::protobuf::Repeated::from_inner(
                ::protobuf::__internal::Private,
                ::protobuf::__internal::runtime::InnerRepeated::from_raw_parts(
                    ::protobuf::__internal::runtime::upb_Array_New(
                        arena.raw(),
                        ::protobuf::__internal::runtime::CType::Message,
                    ),
                    arena,
                ),
            )
        }
    }
    unsafe fn repeated_free(
        _private: ::protobuf::__internal::Private,
        _f: &mut ::protobuf::Repeated<Self>,
    ) {}
    fn repeated_len(f: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
        unsafe {
            ::protobuf::__internal::runtime::upb_Array_Size(
                f.as_raw(::protobuf::__internal::Private),
            )
        }
    }
    unsafe fn repeated_set_unchecked(
        mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
        i: usize,
        v: impl ::protobuf::IntoProxied<Self>,
    ) {
        unsafe {
            ::protobuf::__internal::runtime::upb_Array_Set(
                f.as_raw(::protobuf::__internal::Private),
                i,
                <Self as ::protobuf::__internal::runtime::UpbTypeConversions>::into_message_value_fuse_if_required(
                    f.raw_arena(::protobuf::__internal::Private),
                    v.into_proxied(::protobuf::__internal::Private),
                ),
            )
        }
    }
    unsafe fn repeated_get_unchecked(
        f: ::protobuf::View<::protobuf::Repeated<Self>>,
        i: usize,
    ) -> ::protobuf::View<Self> {
        let msg_ptr = unsafe {
            ::protobuf::__internal::runtime::upb_Array_Get(
                    f.as_raw(::protobuf::__internal::Private),
                    i,
                )
                .msg_val
        }
            .expect("upb_Array* element should not be NULL.");
        ::protobuf::View::<Self>::new(::protobuf::__internal::Private, msg_ptr)
    }
    unsafe fn repeated_get_mut_unchecked(
        mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
        i: usize,
    ) -> ::protobuf::Mut<Self> {
        let msg_ptr = unsafe {
            ::protobuf::__internal::runtime::upb_Array_GetMutable(
                f.as_raw(::protobuf::__internal::Private),
                i,
            )
        };
        unsafe {
            ::protobuf::Mut::<Self> {
                inner: ::protobuf::__internal::runtime::MutatorMessageRef::from_raw_parts(
                    msg_ptr,
                    f.arena(::protobuf::__internal::Private),
                ),
            }
        }
    }
    fn repeated_clear(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
        unsafe {
            ::protobuf::__internal::runtime::upb_Array_Resize(
                f.as_raw(::protobuf::__internal::Private),
                0,
                f.raw_arena(::protobuf::__internal::Private),
            )
        };
    }
    fn repeated_push(
        mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
        v: impl ::protobuf::IntoProxied<Self>,
    ) {
        unsafe {
            ::protobuf::__internal::runtime::upb_Array_Append(
                f.as_raw(::protobuf::__internal::Private),
                <Self as ::protobuf::__internal::runtime::UpbTypeConversions>::into_message_value_fuse_if_required(
                    f.raw_arena(::protobuf::__internal::Private),
                    v.into_proxied(::protobuf::__internal::Private),
                ),
                f.raw_arena(::protobuf::__internal::Private),
            );
        };
    }
    fn repeated_copy_from(
        src: ::protobuf::View<::protobuf::Repeated<Self>>,
        dest: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    ) {
        unsafe {
            ::protobuf::__internal::runtime::repeated_message_copy_from(
                src,
                dest,
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            );
        }
    }
    fn repeated_reserve(
        mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
        additional: usize,
    ) {
        unsafe {
            let size = ::protobuf::__internal::runtime::upb_Array_Size(
                f.as_raw(::protobuf::__internal::Private),
            );
            ::protobuf::__internal::runtime::upb_Array_Reserve(
                f.as_raw(::protobuf::__internal::Private),
                size + additional,
                f.raw_arena(::protobuf::__internal::Private),
            );
        }
    }
}
impl ::protobuf::__internal::runtime::UpbTypeConversions for Rectangle {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }
    fn to_message_value(
        val: ::protobuf::View<'_, Self>,
    ) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue {
            msg_val: Some(val.raw_msg()),
        }
    }
    unsafe fn into_message_value_fuse_if_required(
        raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
        mut val: Self,
    ) -> ::protobuf::__internal::runtime::upb_MessageValue {
        let parent_arena = ::std::mem::ManuallyDrop::new(unsafe {
            ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena)
        });
        parent_arena
            .fuse(val.as_mutator_message_ref(::protobuf::__internal::Private).arena());
        ::protobuf::__internal::runtime::upb_MessageValue {
            msg_val: Some(val.raw_msg()),
        }
    }
    unsafe fn from_message_value<'msg>(
        msg: ::protobuf::__internal::runtime::upb_MessageValue,
    ) -> ::protobuf::View<'msg, Self> {
        RectangleView::new(
            ::protobuf::__internal::Private,
            unsafe { msg.msg_val }.expect("expected present message value in map"),
        )
    }
    unsafe fn from_message_mut<'msg>(
        msg: *mut ::protobuf::__internal::runtime::upb_Message,
        arena: &'msg ::protobuf::__internal::runtime::Arena,
    ) -> RectangleMut<'msg> {
        RectangleMut {
            inner: unsafe {
                ::protobuf::__internal::runtime::MutatorMessageRef::from_raw_parts(
                    std::ptr::NonNull::new(msg)
                        .expect("expected present message value in map"),
                    arena,
                )
            },
        }
    }
}
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RectangleMut<'msg> {
    inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for RectangleMut<'msg> {}
impl<'msg> ::protobuf::MessageMut<'msg> for RectangleMut<'msg> {
    type Message = Rectangle;
}
impl ::std::fmt::Debug for RectangleMut<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for RectangleMut<'_> {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        ::protobuf::AsView::as_view(self).serialize()
    }
}
impl ::protobuf::Clear for RectangleMut<'_> {
    fn clear(&mut self) {
        unsafe {
            ::protobuf::__internal::runtime::upb_Message_Clear(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        }
    }
}
impl ::protobuf::ClearAndParse for RectangleMut<'_> {
    fn clear_and_parse(
        &mut self,
        data: &[u8],
    ) -> ::std::result::Result<(), ::protobuf::ParseError> {
        ::protobuf::Clear::clear(self);
        let status = unsafe {
            ::protobuf::__internal::runtime::wire::decode(
                data,
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                self.arena(),
            )
        };
        match status {
            Ok(_) => Ok(()),
            Err(_) => Err(::protobuf::ParseError),
        }
    }
}
impl ::protobuf::TakeFrom for RectangleMut<'_> {
    fn take_from(&mut self, mut src: impl ::protobuf::AsMut<MutProxied = Rectangle>) {
        let mut src = src.as_mut();
        ::protobuf::CopyFrom::copy_from(self, ::protobuf::AsView::as_view(&src));
        ::protobuf::Clear::clear(&mut src);
    }
}
impl ::protobuf::CopyFrom for RectangleMut<'_> {
    fn copy_from(&mut self, src: impl ::protobuf::AsView<Proxied = Rectangle>) {
        unsafe {
            assert!(
                ::protobuf::__internal::runtime::upb_Message_DeepCopy(self.raw_msg(), src
                .as_view().raw_msg(), < Self as
                ::protobuf::__internal::runtime::AssociatedMiniTable >::mini_table(),
                self.arena().raw())
            );
        }
    }
}
impl ::protobuf::MergeFrom for RectangleMut<'_> {
    fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = Rectangle>) {
        unsafe {
            assert!(
                ::protobuf::__internal::runtime::upb_Message_MergeFrom(self.raw_msg(),
                src.as_view().raw_msg(), < Self as
                ::protobuf::__internal::runtime::AssociatedMiniTable >::mini_table(),
                ::std::ptr::null(), self.arena().raw())
            );
        }
    }
}
#[allow(dead_code)]
impl<'msg> RectangleMut<'msg> {
    #[doc(hidden)]
    pub fn from_parent(
        _private: ::protobuf::__internal::Private,
        parent: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
        msg: ::protobuf::__internal::runtime::RawMessage,
    ) -> Self {
        Self {
            inner: ::protobuf::__internal::runtime::MutatorMessageRef::from_parent(
                parent,
                msg,
            ),
        }
    }
    #[doc(hidden)]
    pub fn new(
        _private: ::protobuf::__internal::Private,
        msg: &'msg mut ::protobuf::__internal::runtime::MessageInner,
    ) -> Self {
        Self {
            inner: ::protobuf::__internal::runtime::MutatorMessageRef::new(msg),
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.msg()
    }
    #[doc(hidden)]
    pub fn as_mutator_message_ref(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MutatorMessageRef<'msg> {
        self.inner
    }
    pub fn to_owned(&self) -> Rectangle {
        ::protobuf::AsView::as_view(self).to_owned()
    }
    fn arena(&self) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
    pub fn has_lo(&self) -> bool {
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_HasBaseField(self.raw_msg(), f)
        }
    }
    pub fn clear_lo(&mut self) {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_ClearBaseField(
                self.raw_msg(),
                f,
            );
        }
    }
    pub fn lo_opt(&self) -> ::protobuf::Optional<super::PointView<'_>> {
        ::protobuf::Optional::new(self.lo(), self.has_lo())
    }
    pub fn lo(&self) -> super::PointView<'_> {
        let submsg = unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_GetMessage(self.raw_msg(), f)
        };
        match submsg {
            None => {
                super::PointView::new(
                    ::protobuf::__internal::Private,
                    ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
                )
            }
            Some(sub_raw_msg) => {
                super::PointView::new(::protobuf::__internal::Private, sub_raw_msg)
            }
        }
    }
    pub fn lo_mut(&mut self) -> super::PointMut<'_> {
        let raw_msg = unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_GetOrCreateMutableMessage(
                    self.raw_msg(),
                    mt,
                    f,
                    self.arena().raw(),
                )
                .unwrap()
        };
        super::PointMut::from_parent(
            ::protobuf::__internal::Private,
            self.as_mutator_message_ref(::protobuf::__internal::Private),
            raw_msg,
        )
    }
    pub fn set_lo(&mut self, val: impl ::protobuf::IntoProxied<super::Point>) {
        let mut msg = val.into_proxied(::protobuf::__internal::Private);
        self.as_mutator_message_ref(::protobuf::__internal::Private)
            .arena()
            .fuse(msg.as_mutator_message_ref(::protobuf::__internal::Private).arena());
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldMessage(
                self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
                f,
                msg.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
            );
        }
    }
    pub fn has_hi(&self) -> bool {
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_HasBaseField(self.raw_msg(), f)
        }
    }
    pub fn clear_hi(&mut self) {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_ClearBaseField(
                self.raw_msg(),
                f,
            );
        }
    }
    pub fn hi_opt(&self) -> ::protobuf::Optional<super::PointView<'_>> {
        ::protobuf::Optional::new(self.hi(), self.has_hi())
    }
    pub fn hi(&self) -> super::PointView<'_> {
        let submsg = unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_GetMessage(self.raw_msg(), f)
        };
        match submsg {
            None => {
                super::PointView::new(
                    ::protobuf::__internal::Private,
                    ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
                )
            }
            Some(sub_raw_msg) => {
                super::PointView::new(::protobuf::__internal::Private, sub_raw_msg)
            }
        }
    }
    pub fn hi_mut(&mut self) -> super::PointMut<'_> {
        let raw_msg = unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_GetOrCreateMutableMessage(
                    self.raw_msg(),
                    mt,
                    f,
                    self.arena().raw(),
                )
                .unwrap()
        };
        super::PointMut::from_parent(
            ::protobuf::__internal::Private,
            self.as_mutator_message_ref(::protobuf::__internal::Private),
            raw_msg,
        )
    }
    pub fn set_hi(&mut self, val: impl ::protobuf::IntoProxied<super::Point>) {
        let mut msg = val.into_proxied(::protobuf::__internal::Private);
        self.as_mutator_message_ref(::protobuf::__internal::Private)
            .arena()
            .fuse(msg.as_mutator_message_ref(::protobuf::__internal::Private).arena());
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldMessage(
                self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
                f,
                msg.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
            );
        }
    }
}
unsafe impl Sync for RectangleMut<'_> {}
impl<'msg> ::protobuf::Proxy<'msg> for RectangleMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for RectangleMut<'msg> {}
impl<'msg> ::protobuf::AsView for RectangleMut<'msg> {
    type Proxied = Rectangle;
    fn as_view(&self) -> ::protobuf::View<'_, Rectangle> {
        RectangleView {
            msg: self.raw_msg(),
            _phantom: ::std::marker::PhantomData,
        }
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for RectangleMut<'msg> {
    fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Rectangle>
    where
        'msg: 'shorter,
    {
        RectangleView {
            msg: self.raw_msg(),
            _phantom: ::std::marker::PhantomData,
        }
    }
}
impl<'msg> ::protobuf::AsMut for RectangleMut<'msg> {
    type MutProxied = Rectangle;
    fn as_mut(&mut self) -> RectangleMut<'msg> {
        RectangleMut { inner: self.inner }
    }
}
impl<'msg> ::protobuf::IntoMut<'msg> for RectangleMut<'msg> {
    fn into_mut<'shorter>(self) -> RectangleMut<'shorter>
    where
        'msg: 'shorter,
    {
        self
    }
}
#[allow(dead_code)]
impl Rectangle {
    pub fn new() -> Self {
        let arena = ::protobuf::__internal::runtime::Arena::new();
        let raw_msg = unsafe {
            ::protobuf::__internal::runtime::upb_Message_New(
                    <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                    arena.raw(),
                )
                .unwrap()
        };
        Self {
            inner: ::protobuf::__internal::runtime::MessageInner {
                msg: raw_msg,
                arena,
            },
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.msg
    }
    #[doc(hidden)]
    pub fn as_mutator_message_ref(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MutatorMessageRef {
        ::protobuf::__internal::runtime::MutatorMessageRef::new(&mut self.inner)
    }
    fn arena(&self) -> &::protobuf::__internal::runtime::Arena {
        &self.inner.arena
    }
    pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        let mut msg = Self::new();
        ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
    }
    pub fn as_view(&self) -> RectangleView {
        RectangleView::new(::protobuf::__internal::Private, self.inner.msg)
    }
    pub fn as_mut(&mut self) -> RectangleMut {
        RectangleMut::new(::protobuf::__internal::Private, &mut self.inner)
    }
    pub fn has_lo(&self) -> bool {
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_HasBaseField(self.raw_msg(), f)
        }
    }
    pub fn clear_lo(&mut self) {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_ClearBaseField(
                self.raw_msg(),
                f,
            );
        }
    }
    pub fn lo_opt(&self) -> ::protobuf::Optional<super::PointView<'_>> {
        ::protobuf::Optional::new(self.lo(), self.has_lo())
    }
    pub fn lo(&self) -> super::PointView<'_> {
        let submsg = unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_GetMessage(self.raw_msg(), f)
        };
        match submsg {
            None => {
                super::PointView::new(
                    ::protobuf::__internal::Private,
                    ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
                )
            }
            Some(sub_raw_msg) => {
                super::PointView::new(::protobuf::__internal::Private, sub_raw_msg)
            }
        }
    }
    pub fn lo_mut(&mut self) -> super::PointMut<'_> {
        let raw_msg = unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_GetOrCreateMutableMessage(
                    self.raw_msg(),
                    mt,
                    f,
                    self.arena().raw(),
                )
                .unwrap()
        };
        super::PointMut::from_parent(
            ::protobuf::__internal::Private,
            self.as_mutator_message_ref(::protobuf::__internal::Private),
            raw_msg,
        )
    }
    pub fn set_lo(&mut self, val: impl ::protobuf::IntoProxied<super::Point>) {
        let mut msg = val.into_proxied(::protobuf::__internal::Private);
        self.as_mutator_message_ref(::protobuf::__internal::Private)
            .arena()
            .fuse(msg.as_mutator_message_ref(::protobuf::__internal::Private).arena());
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldMessage(
                self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
                f,
                msg.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
            );
        }
    }
    pub fn has_hi(&self) -> bool {
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_HasBaseField(self.raw_msg(), f)
        }
    }
    pub fn clear_hi(&mut self) {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_ClearBaseField(
                self.raw_msg(),
                f,
            );
        }
    }
    pub fn hi_opt(&self) -> ::protobuf::Optional<super::PointView<'_>> {
        ::protobuf::Optional::new(self.hi(), self.has_hi())
    }
    pub fn hi(&self) -> super::PointView<'_> {
        let submsg = unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_GetMessage(self.raw_msg(), f)
        };
        match submsg {
            None => {
                super::PointView::new(
                    ::protobuf::__internal::Private,
                    ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
                )
            }
            Some(sub_raw_msg) => {
                super::PointView::new(::protobuf::__internal::Private, sub_raw_msg)
            }
        }
    }
    pub fn hi_mut(&mut self) -> super::PointMut<'_> {
        let raw_msg = unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_GetOrCreateMutableMessage(
                    self.raw_msg(),
                    mt,
                    f,
                    self.arena().raw(),
                )
                .unwrap()
        };
        super::PointMut::from_parent(
            ::protobuf::__internal::Private,
            self.as_mutator_message_ref(::protobuf::__internal::Private),
            raw_msg,
        )
    }
    pub fn set_hi(&mut self, val: impl ::protobuf::IntoProxied<super::Point>) {
        let mut msg = val.into_proxied(::protobuf::__internal::Private);
        self.as_mutator_message_ref(::protobuf::__internal::Private)
            .arena()
            .fuse(msg.as_mutator_message_ref(::protobuf::__internal::Private).arena());
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldMessage(
                self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
                f,
                msg.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
            );
        }
    }
}
impl ::std::ops::Drop for Rectangle {
    fn drop(&mut self) {}
}
impl ::std::clone::Clone for Rectangle {
    fn clone(&self) -> Self {
        self.as_view().to_owned()
    }
}
impl ::protobuf::AsView for Rectangle {
    type Proxied = Self;
    fn as_view(&self) -> RectangleView {
        self.as_view()
    }
}
impl ::protobuf::AsMut for Rectangle {
    type MutProxied = Self;
    fn as_mut(&mut self) -> RectangleMut {
        self.as_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Rectangle {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        #[allow(unused_unsafe)]
        unsafe { ::std::ptr::addr_of!(routeguide__Rectangle_msg_init) }
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RectangleView<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        #[allow(unused_unsafe)]
        unsafe { ::std::ptr::addr_of!(routeguide__Rectangle_msg_init) }
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RectangleMut<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        #[allow(unused_unsafe)]
        unsafe { ::std::ptr::addr_of!(routeguide__Rectangle_msg_init) }
    }
}
extern "C" {
    /// Opaque static extern for this message's MiniTable, generated
    /// by the upb C MiniTable codegen. The only valid way to
    /// reference this static is with `std::ptr::addr_of!(..)`.
    static routeguide__Rectangle_msg_init: ::protobuf::__internal::runtime::upb_MiniTable;
}
impl ::protobuf::OwnedMessageInterop for Rectangle {}
impl<'a> ::protobuf::MessageMutInterop<'a> for RectangleMut<'a> {}
impl<'a> ::protobuf::MessageViewInterop<'a> for RectangleView<'a> {
    unsafe fn __unstable_wrap_raw_message(msg: &'a *const ::std::ffi::c_void) -> Self {
        Self::new(
            ::protobuf::__internal::Private,
            ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap(),
        )
    }
    unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
        msg: *const ::std::ffi::c_void,
    ) -> Self {
        Self::new(
            ::protobuf::__internal::Private,
            ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap(),
        )
    }
    fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
        self.msg.as_ptr() as *const _
    }
}
impl ::protobuf::__internal::MatcherEq for Rectangle {
    fn matches(&self, o: &Self) -> bool {
        ::protobuf::__internal::MatcherEq::matches(
            &::protobuf::AsView::as_view(self),
            &::protobuf::AsView::as_view(o),
        )
    }
}
impl<'a> ::protobuf::__internal::MatcherEq for RectangleMut<'a> {
    fn matches(&self, o: &Self) -> bool {
        ::protobuf::__internal::MatcherEq::matches(
            &::protobuf::AsView::as_view(self),
            &::protobuf::AsView::as_view(o),
        )
    }
}
impl<'a> ::protobuf::__internal::MatcherEq for RectangleView<'a> {
    fn matches(&self, o: &Self) -> bool {
        unsafe {
            ::protobuf::__internal::runtime::upb_Message_IsEqual(
                self.msg,
                o.msg,
                <Rectangle as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            )
        }
    }
}
#[allow(non_camel_case_types)]
pub struct Feature {
    inner: ::protobuf::__internal::runtime::MessageInner,
}
impl ::protobuf::Message for Feature {}
impl ::std::default::Default for Feature {
    fn default() -> Self {
        Self::new()
    }
}
impl ::protobuf::Parse for Feature {
    fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        Self::parse(serialized)
    }
}
impl ::std::fmt::Debug for Feature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::TakeFrom for Feature {
    fn take_from(&mut self, src: impl ::protobuf::AsMut<MutProxied = Self>) {
        let mut m = self.as_mut();
        ::protobuf::TakeFrom::take_from(&mut m, src)
    }
}
impl ::protobuf::CopyFrom for Feature {
    fn copy_from(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
        let mut m = self.as_mut();
        ::protobuf::CopyFrom::copy_from(&mut m, src)
    }
}
impl ::protobuf::MergeFrom for Feature {
    fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
        let mut m = self.as_mut();
        ::protobuf::MergeFrom::merge_from(&mut m, src)
    }
}
impl ::protobuf::Serialize for Feature {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        ::protobuf::AsView::as_view(self).serialize()
    }
}
impl ::protobuf::Clear for Feature {
    fn clear(&mut self) {
        let mut m = self.as_mut();
        ::protobuf::Clear::clear(&mut m)
    }
}
impl ::protobuf::ClearAndParse for Feature {
    fn clear_and_parse(
        &mut self,
        data: &[u8],
    ) -> ::std::result::Result<(), ::protobuf::ParseError> {
        let mut m = self.as_mut();
        ::protobuf::ClearAndParse::clear_and_parse(&mut m, data)
    }
}
unsafe impl Sync for Feature {}
unsafe impl Send for Feature {}
impl ::protobuf::Proxied for Feature {
    type View<'msg> = FeatureView<'msg>;
}
impl ::protobuf::__internal::SealedInternal for Feature {}
impl ::protobuf::MutProxied for Feature {
    type Mut<'msg> = FeatureMut<'msg>;
}
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FeatureView<'msg> {
    msg: ::protobuf::__internal::runtime::RawMessage,
    _phantom: ::std::marker::PhantomData<&'msg ()>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for FeatureView<'msg> {}
impl<'msg> ::protobuf::MessageView<'msg> for FeatureView<'msg> {
    type Message = Feature;
}
impl ::std::fmt::Debug for FeatureView<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for FeatureView<'_> {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        let encoded = unsafe {
            ::protobuf::__internal::runtime::wire::encode(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        encoded.map_err(|_| ::protobuf::SerializeError)
    }
}
impl ::std::default::Default for FeatureView<'_> {
    fn default() -> FeatureView<'static> {
        FeatureView::new(
            ::protobuf::__internal::Private,
            ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
        )
    }
}
#[allow(dead_code)]
impl<'msg> FeatureView<'msg> {
    #[doc(hidden)]
    pub fn new(
        _private: ::protobuf::__internal::Private,
        msg: ::protobuf::__internal::runtime::RawMessage,
    ) -> Self {
        Self {
            msg,
            _phantom: ::std::marker::PhantomData,
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.msg
    }
    pub fn to_owned(&self) -> Feature {
        ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
    }
    pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
        let str_view = unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_GetString(
                self.raw_msg(),
                f,
                (b"").into(),
            )
        };
        unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
    }
    pub fn has_location(self) -> bool {
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_HasBaseField(self.raw_msg(), f)
        }
    }
    pub fn location_opt(self) -> ::protobuf::Optional<super::PointView<'msg>> {
        ::protobuf::Optional::new(self.location(), self.has_location())
    }
    pub fn location(self) -> super::PointView<'msg> {
        let submsg = unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_GetMessage(self.raw_msg(), f)
        };
        match submsg {
            None => {
                super::PointView::new(
                    ::protobuf::__internal::Private,
                    ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
                )
            }
            Some(sub_raw_msg) => {
                super::PointView::new(::protobuf::__internal::Private, sub_raw_msg)
            }
        }
    }
}
unsafe impl Sync for FeatureView<'_> {}
unsafe impl Send for FeatureView<'_> {}
impl<'msg> ::protobuf::Proxy<'msg> for FeatureView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for FeatureView<'msg> {}
impl<'msg> ::protobuf::AsView for FeatureView<'msg> {
    type Proxied = Feature;
    fn as_view(&self) -> ::protobuf::View<'msg, Feature> {
        *self
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for FeatureView<'msg> {
    fn into_view<'shorter>(self) -> FeatureView<'shorter>
    where
        'msg: 'shorter,
    {
        self
    }
}
impl<'msg> ::protobuf::IntoProxied<Feature> for FeatureView<'msg> {
    fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Feature {
        let dst = Feature::new();
        unsafe {
            ::protobuf::__internal::runtime::upb_Message_DeepCopy(
                dst.inner.msg,
                self.msg,
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                dst.inner.arena.raw(),
            )
        };
        dst
    }
}
impl<'msg> ::protobuf::IntoProxied<Feature> for FeatureMut<'msg> {
    fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Feature {
        ::protobuf::IntoProxied::into_proxied(
            ::protobuf::IntoView::into_view(self),
            _private,
        )
    }
}
unsafe impl ::protobuf::ProxiedInRepeated for Feature {
    fn repeated_new(
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::Repeated<Self> {
        let arena = ::protobuf::__internal::runtime::Arena::new();
        unsafe {
            ::protobuf::Repeated::from_inner(
                ::protobuf::__internal::Private,
                ::protobuf::__internal::runtime::InnerRepeated::from_raw_parts(
                    ::protobuf::__internal::runtime::upb_Array_New(
                        arena.raw(),
                        ::protobuf::__internal::runtime::CType::Message,
                    ),
                    arena,
                ),
            )
        }
    }
    unsafe fn repeated_free(
        _private: ::protobuf::__internal::Private,
        _f: &mut ::protobuf::Repeated<Self>,
    ) {}
    fn repeated_len(f: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
        unsafe {
            ::protobuf::__internal::runtime::upb_Array_Size(
                f.as_raw(::protobuf::__internal::Private),
            )
        }
    }
    unsafe fn repeated_set_unchecked(
        mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
        i: usize,
        v: impl ::protobuf::IntoProxied<Self>,
    ) {
        unsafe {
            ::protobuf::__internal::runtime::upb_Array_Set(
                f.as_raw(::protobuf::__internal::Private),
                i,
                <Self as ::protobuf::__internal::runtime::UpbTypeConversions>::into_message_value_fuse_if_required(
                    f.raw_arena(::protobuf::__internal::Private),
                    v.into_proxied(::protobuf::__internal::Private),
                ),
            )
        }
    }
    unsafe fn repeated_get_unchecked(
        f: ::protobuf::View<::protobuf::Repeated<Self>>,
        i: usize,
    ) -> ::protobuf::View<Self> {
        let msg_ptr = unsafe {
            ::protobuf::__internal::runtime::upb_Array_Get(
                    f.as_raw(::protobuf::__internal::Private),
                    i,
                )
                .msg_val
        }
            .expect("upb_Array* element should not be NULL.");
        ::protobuf::View::<Self>::new(::protobuf::__internal::Private, msg_ptr)
    }
    unsafe fn repeated_get_mut_unchecked(
        mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
        i: usize,
    ) -> ::protobuf::Mut<Self> {
        let msg_ptr = unsafe {
            ::protobuf::__internal::runtime::upb_Array_GetMutable(
                f.as_raw(::protobuf::__internal::Private),
                i,
            )
        };
        unsafe {
            ::protobuf::Mut::<Self> {
                inner: ::protobuf::__internal::runtime::MutatorMessageRef::from_raw_parts(
                    msg_ptr,
                    f.arena(::protobuf::__internal::Private),
                ),
            }
        }
    }
    fn repeated_clear(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
        unsafe {
            ::protobuf::__internal::runtime::upb_Array_Resize(
                f.as_raw(::protobuf::__internal::Private),
                0,
                f.raw_arena(::protobuf::__internal::Private),
            )
        };
    }
    fn repeated_push(
        mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
        v: impl ::protobuf::IntoProxied<Self>,
    ) {
        unsafe {
            ::protobuf::__internal::runtime::upb_Array_Append(
                f.as_raw(::protobuf::__internal::Private),
                <Self as ::protobuf::__internal::runtime::UpbTypeConversions>::into_message_value_fuse_if_required(
                    f.raw_arena(::protobuf::__internal::Private),
                    v.into_proxied(::protobuf::__internal::Private),
                ),
                f.raw_arena(::protobuf::__internal::Private),
            );
        };
    }
    fn repeated_copy_from(
        src: ::protobuf::View<::protobuf::Repeated<Self>>,
        dest: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    ) {
        unsafe {
            ::protobuf::__internal::runtime::repeated_message_copy_from(
                src,
                dest,
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            );
        }
    }
    fn repeated_reserve(
        mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
        additional: usize,
    ) {
        unsafe {
            let size = ::protobuf::__internal::runtime::upb_Array_Size(
                f.as_raw(::protobuf::__internal::Private),
            );
            ::protobuf::__internal::runtime::upb_Array_Reserve(
                f.as_raw(::protobuf::__internal::Private),
                size + additional,
                f.raw_arena(::protobuf::__internal::Private),
            );
        }
    }
}
impl ::protobuf::__internal::runtime::UpbTypeConversions for Feature {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }
    fn to_message_value(
        val: ::protobuf::View<'_, Self>,
    ) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue {
            msg_val: Some(val.raw_msg()),
        }
    }
    unsafe fn into_message_value_fuse_if_required(
        raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
        mut val: Self,
    ) -> ::protobuf::__internal::runtime::upb_MessageValue {
        let parent_arena = ::std::mem::ManuallyDrop::new(unsafe {
            ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena)
        });
        parent_arena
            .fuse(val.as_mutator_message_ref(::protobuf::__internal::Private).arena());
        ::protobuf::__internal::runtime::upb_MessageValue {
            msg_val: Some(val.raw_msg()),
        }
    }
    unsafe fn from_message_value<'msg>(
        msg: ::protobuf::__internal::runtime::upb_MessageValue,
    ) -> ::protobuf::View<'msg, Self> {
        FeatureView::new(
            ::protobuf::__internal::Private,
            unsafe { msg.msg_val }.expect("expected present message value in map"),
        )
    }
    unsafe fn from_message_mut<'msg>(
        msg: *mut ::protobuf::__internal::runtime::upb_Message,
        arena: &'msg ::protobuf::__internal::runtime::Arena,
    ) -> FeatureMut<'msg> {
        FeatureMut {
            inner: unsafe {
                ::protobuf::__internal::runtime::MutatorMessageRef::from_raw_parts(
                    std::ptr::NonNull::new(msg)
                        .expect("expected present message value in map"),
                    arena,
                )
            },
        }
    }
}
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FeatureMut<'msg> {
    inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for FeatureMut<'msg> {}
impl<'msg> ::protobuf::MessageMut<'msg> for FeatureMut<'msg> {
    type Message = Feature;
}
impl ::std::fmt::Debug for FeatureMut<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for FeatureMut<'_> {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        ::protobuf::AsView::as_view(self).serialize()
    }
}
impl ::protobuf::Clear for FeatureMut<'_> {
    fn clear(&mut self) {
        unsafe {
            ::protobuf::__internal::runtime::upb_Message_Clear(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        }
    }
}
impl ::protobuf::ClearAndParse for FeatureMut<'_> {
    fn clear_and_parse(
        &mut self,
        data: &[u8],
    ) -> ::std::result::Result<(), ::protobuf::ParseError> {
        ::protobuf::Clear::clear(self);
        let status = unsafe {
            ::protobuf::__internal::runtime::wire::decode(
                data,
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                self.arena(),
            )
        };
        match status {
            Ok(_) => Ok(()),
            Err(_) => Err(::protobuf::ParseError),
        }
    }
}
impl ::protobuf::TakeFrom for FeatureMut<'_> {
    fn take_from(&mut self, mut src: impl ::protobuf::AsMut<MutProxied = Feature>) {
        let mut src = src.as_mut();
        ::protobuf::CopyFrom::copy_from(self, ::protobuf::AsView::as_view(&src));
        ::protobuf::Clear::clear(&mut src);
    }
}
impl ::protobuf::CopyFrom for FeatureMut<'_> {
    fn copy_from(&mut self, src: impl ::protobuf::AsView<Proxied = Feature>) {
        unsafe {
            assert!(
                ::protobuf::__internal::runtime::upb_Message_DeepCopy(self.raw_msg(), src
                .as_view().raw_msg(), < Self as
                ::protobuf::__internal::runtime::AssociatedMiniTable >::mini_table(),
                self.arena().raw())
            );
        }
    }
}
impl ::protobuf::MergeFrom for FeatureMut<'_> {
    fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = Feature>) {
        unsafe {
            assert!(
                ::protobuf::__internal::runtime::upb_Message_MergeFrom(self.raw_msg(),
                src.as_view().raw_msg(), < Self as
                ::protobuf::__internal::runtime::AssociatedMiniTable >::mini_table(),
                ::std::ptr::null(), self.arena().raw())
            );
        }
    }
}
#[allow(dead_code)]
impl<'msg> FeatureMut<'msg> {
    #[doc(hidden)]
    pub fn from_parent(
        _private: ::protobuf::__internal::Private,
        parent: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
        msg: ::protobuf::__internal::runtime::RawMessage,
    ) -> Self {
        Self {
            inner: ::protobuf::__internal::runtime::MutatorMessageRef::from_parent(
                parent,
                msg,
            ),
        }
    }
    #[doc(hidden)]
    pub fn new(
        _private: ::protobuf::__internal::Private,
        msg: &'msg mut ::protobuf::__internal::runtime::MessageInner,
    ) -> Self {
        Self {
            inner: ::protobuf::__internal::runtime::MutatorMessageRef::new(msg),
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.msg()
    }
    #[doc(hidden)]
    pub fn as_mutator_message_ref(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MutatorMessageRef<'msg> {
        self.inner
    }
    pub fn to_owned(&self) -> Feature {
        ::protobuf::AsView::as_view(self).to_owned()
    }
    fn arena(&self) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
    pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
        let str_view = unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_GetString(
                self.raw_msg(),
                f,
                (b"").into(),
            )
        };
        unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
    }
    pub fn set_name(
        &mut self,
        val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>,
    ) {
        let s = val.into_proxied(::protobuf::__internal::Private);
        let (view, arena) = s
            .into_inner(::protobuf::__internal::Private)
            .into_raw_parts();
        let mm_ref = self.as_mutator_message_ref(::protobuf::__internal::Private);
        let parent_arena = mm_ref.arena();
        parent_arena.fuse(&arena);
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldString(
                self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
                f,
                view,
            );
        }
    }
    pub fn has_location(&self) -> bool {
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_HasBaseField(self.raw_msg(), f)
        }
    }
    pub fn clear_location(&mut self) {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_ClearBaseField(
                self.raw_msg(),
                f,
            );
        }
    }
    pub fn location_opt(&self) -> ::protobuf::Optional<super::PointView<'_>> {
        ::protobuf::Optional::new(self.location(), self.has_location())
    }
    pub fn location(&self) -> super::PointView<'_> {
        let submsg = unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_GetMessage(self.raw_msg(), f)
        };
        match submsg {
            None => {
                super::PointView::new(
                    ::protobuf::__internal::Private,
                    ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
                )
            }
            Some(sub_raw_msg) => {
                super::PointView::new(::protobuf::__internal::Private, sub_raw_msg)
            }
        }
    }
    pub fn location_mut(&mut self) -> super::PointMut<'_> {
        let raw_msg = unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_GetOrCreateMutableMessage(
                    self.raw_msg(),
                    mt,
                    f,
                    self.arena().raw(),
                )
                .unwrap()
        };
        super::PointMut::from_parent(
            ::protobuf::__internal::Private,
            self.as_mutator_message_ref(::protobuf::__internal::Private),
            raw_msg,
        )
    }
    pub fn set_location(&mut self, val: impl ::protobuf::IntoProxied<super::Point>) {
        let mut msg = val.into_proxied(::protobuf::__internal::Private);
        self.as_mutator_message_ref(::protobuf::__internal::Private)
            .arena()
            .fuse(msg.as_mutator_message_ref(::protobuf::__internal::Private).arena());
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldMessage(
                self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
                f,
                msg.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
            );
        }
    }
}
unsafe impl Sync for FeatureMut<'_> {}
impl<'msg> ::protobuf::Proxy<'msg> for FeatureMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for FeatureMut<'msg> {}
impl<'msg> ::protobuf::AsView for FeatureMut<'msg> {
    type Proxied = Feature;
    fn as_view(&self) -> ::protobuf::View<'_, Feature> {
        FeatureView {
            msg: self.raw_msg(),
            _phantom: ::std::marker::PhantomData,
        }
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for FeatureMut<'msg> {
    fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Feature>
    where
        'msg: 'shorter,
    {
        FeatureView {
            msg: self.raw_msg(),
            _phantom: ::std::marker::PhantomData,
        }
    }
}
impl<'msg> ::protobuf::AsMut for FeatureMut<'msg> {
    type MutProxied = Feature;
    fn as_mut(&mut self) -> FeatureMut<'msg> {
        FeatureMut { inner: self.inner }
    }
}
impl<'msg> ::protobuf::IntoMut<'msg> for FeatureMut<'msg> {
    fn into_mut<'shorter>(self) -> FeatureMut<'shorter>
    where
        'msg: 'shorter,
    {
        self
    }
}
#[allow(dead_code)]
impl Feature {
    pub fn new() -> Self {
        let arena = ::protobuf::__internal::runtime::Arena::new();
        let raw_msg = unsafe {
            ::protobuf::__internal::runtime::upb_Message_New(
                    <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                    arena.raw(),
                )
                .unwrap()
        };
        Self {
            inner: ::protobuf::__internal::runtime::MessageInner {
                msg: raw_msg,
                arena,
            },
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.msg
    }
    #[doc(hidden)]
    pub fn as_mutator_message_ref(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MutatorMessageRef {
        ::protobuf::__internal::runtime::MutatorMessageRef::new(&mut self.inner)
    }
    fn arena(&self) -> &::protobuf::__internal::runtime::Arena {
        &self.inner.arena
    }
    pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        let mut msg = Self::new();
        ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
    }
    pub fn as_view(&self) -> FeatureView {
        FeatureView::new(::protobuf::__internal::Private, self.inner.msg)
    }
    pub fn as_mut(&mut self) -> FeatureMut {
        FeatureMut::new(::protobuf::__internal::Private, &mut self.inner)
    }
    pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
        let str_view = unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_GetString(
                self.raw_msg(),
                f,
                (b"").into(),
            )
        };
        unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
    }
    pub fn set_name(
        &mut self,
        val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>,
    ) {
        let s = val.into_proxied(::protobuf::__internal::Private);
        let (view, arena) = s
            .into_inner(::protobuf::__internal::Private)
            .into_raw_parts();
        let mm_ref = self.as_mutator_message_ref(::protobuf::__internal::Private);
        let parent_arena = mm_ref.arena();
        parent_arena.fuse(&arena);
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldString(
                self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
                f,
                view,
            );
        }
    }
    pub fn has_location(&self) -> bool {
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_HasBaseField(self.raw_msg(), f)
        }
    }
    pub fn clear_location(&mut self) {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_ClearBaseField(
                self.raw_msg(),
                f,
            );
        }
    }
    pub fn location_opt(&self) -> ::protobuf::Optional<super::PointView<'_>> {
        ::protobuf::Optional::new(self.location(), self.has_location())
    }
    pub fn location(&self) -> super::PointView<'_> {
        let submsg = unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_GetMessage(self.raw_msg(), f)
        };
        match submsg {
            None => {
                super::PointView::new(
                    ::protobuf::__internal::Private,
                    ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
                )
            }
            Some(sub_raw_msg) => {
                super::PointView::new(::protobuf::__internal::Private, sub_raw_msg)
            }
        }
    }
    pub fn location_mut(&mut self) -> super::PointMut<'_> {
        let raw_msg = unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_GetOrCreateMutableMessage(
                    self.raw_msg(),
                    mt,
                    f,
                    self.arena().raw(),
                )
                .unwrap()
        };
        super::PointMut::from_parent(
            ::protobuf::__internal::Private,
            self.as_mutator_message_ref(::protobuf::__internal::Private),
            raw_msg,
        )
    }
    pub fn set_location(&mut self, val: impl ::protobuf::IntoProxied<super::Point>) {
        let mut msg = val.into_proxied(::protobuf::__internal::Private);
        self.as_mutator_message_ref(::protobuf::__internal::Private)
            .arena()
            .fuse(msg.as_mutator_message_ref(::protobuf::__internal::Private).arena());
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldMessage(
                self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
                f,
                msg.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
            );
        }
    }
}
impl ::std::ops::Drop for Feature {
    fn drop(&mut self) {}
}
impl ::std::clone::Clone for Feature {
    fn clone(&self) -> Self {
        self.as_view().to_owned()
    }
}
impl ::protobuf::AsView for Feature {
    type Proxied = Self;
    fn as_view(&self) -> FeatureView {
        self.as_view()
    }
}
impl ::protobuf::AsMut for Feature {
    type MutProxied = Self;
    fn as_mut(&mut self) -> FeatureMut {
        self.as_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Feature {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        #[allow(unused_unsafe)]
        unsafe { ::std::ptr::addr_of!(routeguide__Feature_msg_init) }
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FeatureView<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        #[allow(unused_unsafe)]
        unsafe { ::std::ptr::addr_of!(routeguide__Feature_msg_init) }
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FeatureMut<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        #[allow(unused_unsafe)]
        unsafe { ::std::ptr::addr_of!(routeguide__Feature_msg_init) }
    }
}
extern "C" {
    /// Opaque static extern for this message's MiniTable, generated
    /// by the upb C MiniTable codegen. The only valid way to
    /// reference this static is with `std::ptr::addr_of!(..)`.
    static routeguide__Feature_msg_init: ::protobuf::__internal::runtime::upb_MiniTable;
}
impl ::protobuf::OwnedMessageInterop for Feature {}
impl<'a> ::protobuf::MessageMutInterop<'a> for FeatureMut<'a> {}
impl<'a> ::protobuf::MessageViewInterop<'a> for FeatureView<'a> {
    unsafe fn __unstable_wrap_raw_message(msg: &'a *const ::std::ffi::c_void) -> Self {
        Self::new(
            ::protobuf::__internal::Private,
            ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap(),
        )
    }
    unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
        msg: *const ::std::ffi::c_void,
    ) -> Self {
        Self::new(
            ::protobuf::__internal::Private,
            ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap(),
        )
    }
    fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
        self.msg.as_ptr() as *const _
    }
}
impl ::protobuf::__internal::MatcherEq for Feature {
    fn matches(&self, o: &Self) -> bool {
        ::protobuf::__internal::MatcherEq::matches(
            &::protobuf::AsView::as_view(self),
            &::protobuf::AsView::as_view(o),
        )
    }
}
impl<'a> ::protobuf::__internal::MatcherEq for FeatureMut<'a> {
    fn matches(&self, o: &Self) -> bool {
        ::protobuf::__internal::MatcherEq::matches(
            &::protobuf::AsView::as_view(self),
            &::protobuf::AsView::as_view(o),
        )
    }
}
impl<'a> ::protobuf::__internal::MatcherEq for FeatureView<'a> {
    fn matches(&self, o: &Self) -> bool {
        unsafe {
            ::protobuf::__internal::runtime::upb_Message_IsEqual(
                self.msg,
                o.msg,
                <Feature as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            )
        }
    }
}
#[allow(non_camel_case_types)]
pub struct RouteNote {
    inner: ::protobuf::__internal::runtime::MessageInner,
}
impl ::protobuf::Message for RouteNote {}
impl ::std::default::Default for RouteNote {
    fn default() -> Self {
        Self::new()
    }
}
impl ::protobuf::Parse for RouteNote {
    fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        Self::parse(serialized)
    }
}
impl ::std::fmt::Debug for RouteNote {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::TakeFrom for RouteNote {
    fn take_from(&mut self, src: impl ::protobuf::AsMut<MutProxied = Self>) {
        let mut m = self.as_mut();
        ::protobuf::TakeFrom::take_from(&mut m, src)
    }
}
impl ::protobuf::CopyFrom for RouteNote {
    fn copy_from(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
        let mut m = self.as_mut();
        ::protobuf::CopyFrom::copy_from(&mut m, src)
    }
}
impl ::protobuf::MergeFrom for RouteNote {
    fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
        let mut m = self.as_mut();
        ::protobuf::MergeFrom::merge_from(&mut m, src)
    }
}
impl ::protobuf::Serialize for RouteNote {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        ::protobuf::AsView::as_view(self).serialize()
    }
}
impl ::protobuf::Clear for RouteNote {
    fn clear(&mut self) {
        let mut m = self.as_mut();
        ::protobuf::Clear::clear(&mut m)
    }
}
impl ::protobuf::ClearAndParse for RouteNote {
    fn clear_and_parse(
        &mut self,
        data: &[u8],
    ) -> ::std::result::Result<(), ::protobuf::ParseError> {
        let mut m = self.as_mut();
        ::protobuf::ClearAndParse::clear_and_parse(&mut m, data)
    }
}
unsafe impl Sync for RouteNote {}
unsafe impl Send for RouteNote {}
impl ::protobuf::Proxied for RouteNote {
    type View<'msg> = RouteNoteView<'msg>;
}
impl ::protobuf::__internal::SealedInternal for RouteNote {}
impl ::protobuf::MutProxied for RouteNote {
    type Mut<'msg> = RouteNoteMut<'msg>;
}
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RouteNoteView<'msg> {
    msg: ::protobuf::__internal::runtime::RawMessage,
    _phantom: ::std::marker::PhantomData<&'msg ()>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for RouteNoteView<'msg> {}
impl<'msg> ::protobuf::MessageView<'msg> for RouteNoteView<'msg> {
    type Message = RouteNote;
}
impl ::std::fmt::Debug for RouteNoteView<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for RouteNoteView<'_> {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        let encoded = unsafe {
            ::protobuf::__internal::runtime::wire::encode(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        encoded.map_err(|_| ::protobuf::SerializeError)
    }
}
impl ::std::default::Default for RouteNoteView<'_> {
    fn default() -> RouteNoteView<'static> {
        RouteNoteView::new(
            ::protobuf::__internal::Private,
            ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
        )
    }
}
#[allow(dead_code)]
impl<'msg> RouteNoteView<'msg> {
    #[doc(hidden)]
    pub fn new(
        _private: ::protobuf::__internal::Private,
        msg: ::protobuf::__internal::runtime::RawMessage,
    ) -> Self {
        Self {
            msg,
            _phantom: ::std::marker::PhantomData,
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.msg
    }
    pub fn to_owned(&self) -> RouteNote {
        ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
    }
    pub fn has_location(self) -> bool {
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_HasBaseField(self.raw_msg(), f)
        }
    }
    pub fn location_opt(self) -> ::protobuf::Optional<super::PointView<'msg>> {
        ::protobuf::Optional::new(self.location(), self.has_location())
    }
    pub fn location(self) -> super::PointView<'msg> {
        let submsg = unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_GetMessage(self.raw_msg(), f)
        };
        match submsg {
            None => {
                super::PointView::new(
                    ::protobuf::__internal::Private,
                    ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
                )
            }
            Some(sub_raw_msg) => {
                super::PointView::new(::protobuf::__internal::Private, sub_raw_msg)
            }
        }
    }
    pub fn message(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
        let str_view = unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_GetString(
                self.raw_msg(),
                f,
                (b"").into(),
            )
        };
        unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
    }
}
unsafe impl Sync for RouteNoteView<'_> {}
unsafe impl Send for RouteNoteView<'_> {}
impl<'msg> ::protobuf::Proxy<'msg> for RouteNoteView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for RouteNoteView<'msg> {}
impl<'msg> ::protobuf::AsView for RouteNoteView<'msg> {
    type Proxied = RouteNote;
    fn as_view(&self) -> ::protobuf::View<'msg, RouteNote> {
        *self
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for RouteNoteView<'msg> {
    fn into_view<'shorter>(self) -> RouteNoteView<'shorter>
    where
        'msg: 'shorter,
    {
        self
    }
}
impl<'msg> ::protobuf::IntoProxied<RouteNote> for RouteNoteView<'msg> {
    fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RouteNote {
        let dst = RouteNote::new();
        unsafe {
            ::protobuf::__internal::runtime::upb_Message_DeepCopy(
                dst.inner.msg,
                self.msg,
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                dst.inner.arena.raw(),
            )
        };
        dst
    }
}
impl<'msg> ::protobuf::IntoProxied<RouteNote> for RouteNoteMut<'msg> {
    fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RouteNote {
        ::protobuf::IntoProxied::into_proxied(
            ::protobuf::IntoView::into_view(self),
            _private,
        )
    }
}
unsafe impl ::protobuf::ProxiedInRepeated for RouteNote {
    fn repeated_new(
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::Repeated<Self> {
        let arena = ::protobuf::__internal::runtime::Arena::new();
        unsafe {
            ::protobuf::Repeated::from_inner(
                ::protobuf::__internal::Private,
                ::protobuf::__internal::runtime::InnerRepeated::from_raw_parts(
                    ::protobuf::__internal::runtime::upb_Array_New(
                        arena.raw(),
                        ::protobuf::__internal::runtime::CType::Message,
                    ),
                    arena,
                ),
            )
        }
    }
    unsafe fn repeated_free(
        _private: ::protobuf::__internal::Private,
        _f: &mut ::protobuf::Repeated<Self>,
    ) {}
    fn repeated_len(f: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
        unsafe {
            ::protobuf::__internal::runtime::upb_Array_Size(
                f.as_raw(::protobuf::__internal::Private),
            )
        }
    }
    unsafe fn repeated_set_unchecked(
        mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
        i: usize,
        v: impl ::protobuf::IntoProxied<Self>,
    ) {
        unsafe {
            ::protobuf::__internal::runtime::upb_Array_Set(
                f.as_raw(::protobuf::__internal::Private),
                i,
                <Self as ::protobuf::__internal::runtime::UpbTypeConversions>::into_message_value_fuse_if_required(
                    f.raw_arena(::protobuf::__internal::Private),
                    v.into_proxied(::protobuf::__internal::Private),
                ),
            )
        }
    }
    unsafe fn repeated_get_unchecked(
        f: ::protobuf::View<::protobuf::Repeated<Self>>,
        i: usize,
    ) -> ::protobuf::View<Self> {
        let msg_ptr = unsafe {
            ::protobuf::__internal::runtime::upb_Array_Get(
                    f.as_raw(::protobuf::__internal::Private),
                    i,
                )
                .msg_val
        }
            .expect("upb_Array* element should not be NULL.");
        ::protobuf::View::<Self>::new(::protobuf::__internal::Private, msg_ptr)
    }
    unsafe fn repeated_get_mut_unchecked(
        mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
        i: usize,
    ) -> ::protobuf::Mut<Self> {
        let msg_ptr = unsafe {
            ::protobuf::__internal::runtime::upb_Array_GetMutable(
                f.as_raw(::protobuf::__internal::Private),
                i,
            )
        };
        unsafe {
            ::protobuf::Mut::<Self> {
                inner: ::protobuf::__internal::runtime::MutatorMessageRef::from_raw_parts(
                    msg_ptr,
                    f.arena(::protobuf::__internal::Private),
                ),
            }
        }
    }
    fn repeated_clear(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
        unsafe {
            ::protobuf::__internal::runtime::upb_Array_Resize(
                f.as_raw(::protobuf::__internal::Private),
                0,
                f.raw_arena(::protobuf::__internal::Private),
            )
        };
    }
    fn repeated_push(
        mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
        v: impl ::protobuf::IntoProxied<Self>,
    ) {
        unsafe {
            ::protobuf::__internal::runtime::upb_Array_Append(
                f.as_raw(::protobuf::__internal::Private),
                <Self as ::protobuf::__internal::runtime::UpbTypeConversions>::into_message_value_fuse_if_required(
                    f.raw_arena(::protobuf::__internal::Private),
                    v.into_proxied(::protobuf::__internal::Private),
                ),
                f.raw_arena(::protobuf::__internal::Private),
            );
        };
    }
    fn repeated_copy_from(
        src: ::protobuf::View<::protobuf::Repeated<Self>>,
        dest: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    ) {
        unsafe {
            ::protobuf::__internal::runtime::repeated_message_copy_from(
                src,
                dest,
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            );
        }
    }
    fn repeated_reserve(
        mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
        additional: usize,
    ) {
        unsafe {
            let size = ::protobuf::__internal::runtime::upb_Array_Size(
                f.as_raw(::protobuf::__internal::Private),
            );
            ::protobuf::__internal::runtime::upb_Array_Reserve(
                f.as_raw(::protobuf::__internal::Private),
                size + additional,
                f.raw_arena(::protobuf::__internal::Private),
            );
        }
    }
}
impl ::protobuf::__internal::runtime::UpbTypeConversions for RouteNote {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }
    fn to_message_value(
        val: ::protobuf::View<'_, Self>,
    ) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue {
            msg_val: Some(val.raw_msg()),
        }
    }
    unsafe fn into_message_value_fuse_if_required(
        raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
        mut val: Self,
    ) -> ::protobuf::__internal::runtime::upb_MessageValue {
        let parent_arena = ::std::mem::ManuallyDrop::new(unsafe {
            ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena)
        });
        parent_arena
            .fuse(val.as_mutator_message_ref(::protobuf::__internal::Private).arena());
        ::protobuf::__internal::runtime::upb_MessageValue {
            msg_val: Some(val.raw_msg()),
        }
    }
    unsafe fn from_message_value<'msg>(
        msg: ::protobuf::__internal::runtime::upb_MessageValue,
    ) -> ::protobuf::View<'msg, Self> {
        RouteNoteView::new(
            ::protobuf::__internal::Private,
            unsafe { msg.msg_val }.expect("expected present message value in map"),
        )
    }
    unsafe fn from_message_mut<'msg>(
        msg: *mut ::protobuf::__internal::runtime::upb_Message,
        arena: &'msg ::protobuf::__internal::runtime::Arena,
    ) -> RouteNoteMut<'msg> {
        RouteNoteMut {
            inner: unsafe {
                ::protobuf::__internal::runtime::MutatorMessageRef::from_raw_parts(
                    std::ptr::NonNull::new(msg)
                        .expect("expected present message value in map"),
                    arena,
                )
            },
        }
    }
}
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RouteNoteMut<'msg> {
    inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for RouteNoteMut<'msg> {}
impl<'msg> ::protobuf::MessageMut<'msg> for RouteNoteMut<'msg> {
    type Message = RouteNote;
}
impl ::std::fmt::Debug for RouteNoteMut<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for RouteNoteMut<'_> {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        ::protobuf::AsView::as_view(self).serialize()
    }
}
impl ::protobuf::Clear for RouteNoteMut<'_> {
    fn clear(&mut self) {
        unsafe {
            ::protobuf::__internal::runtime::upb_Message_Clear(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        }
    }
}
impl ::protobuf::ClearAndParse for RouteNoteMut<'_> {
    fn clear_and_parse(
        &mut self,
        data: &[u8],
    ) -> ::std::result::Result<(), ::protobuf::ParseError> {
        ::protobuf::Clear::clear(self);
        let status = unsafe {
            ::protobuf::__internal::runtime::wire::decode(
                data,
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                self.arena(),
            )
        };
        match status {
            Ok(_) => Ok(()),
            Err(_) => Err(::protobuf::ParseError),
        }
    }
}
impl ::protobuf::TakeFrom for RouteNoteMut<'_> {
    fn take_from(&mut self, mut src: impl ::protobuf::AsMut<MutProxied = RouteNote>) {
        let mut src = src.as_mut();
        ::protobuf::CopyFrom::copy_from(self, ::protobuf::AsView::as_view(&src));
        ::protobuf::Clear::clear(&mut src);
    }
}
impl ::protobuf::CopyFrom for RouteNoteMut<'_> {
    fn copy_from(&mut self, src: impl ::protobuf::AsView<Proxied = RouteNote>) {
        unsafe {
            assert!(
                ::protobuf::__internal::runtime::upb_Message_DeepCopy(self.raw_msg(), src
                .as_view().raw_msg(), < Self as
                ::protobuf::__internal::runtime::AssociatedMiniTable >::mini_table(),
                self.arena().raw())
            );
        }
    }
}
impl ::protobuf::MergeFrom for RouteNoteMut<'_> {
    fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = RouteNote>) {
        unsafe {
            assert!(
                ::protobuf::__internal::runtime::upb_Message_MergeFrom(self.raw_msg(),
                src.as_view().raw_msg(), < Self as
                ::protobuf::__internal::runtime::AssociatedMiniTable >::mini_table(),
                ::std::ptr::null(), self.arena().raw())
            );
        }
    }
}
#[allow(dead_code)]
impl<'msg> RouteNoteMut<'msg> {
    #[doc(hidden)]
    pub fn from_parent(
        _private: ::protobuf::__internal::Private,
        parent: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
        msg: ::protobuf::__internal::runtime::RawMessage,
    ) -> Self {
        Self {
            inner: ::protobuf::__internal::runtime::MutatorMessageRef::from_parent(
                parent,
                msg,
            ),
        }
    }
    #[doc(hidden)]
    pub fn new(
        _private: ::protobuf::__internal::Private,
        msg: &'msg mut ::protobuf::__internal::runtime::MessageInner,
    ) -> Self {
        Self {
            inner: ::protobuf::__internal::runtime::MutatorMessageRef::new(msg),
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.msg()
    }
    #[doc(hidden)]
    pub fn as_mutator_message_ref(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MutatorMessageRef<'msg> {
        self.inner
    }
    pub fn to_owned(&self) -> RouteNote {
        ::protobuf::AsView::as_view(self).to_owned()
    }
    fn arena(&self) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
    pub fn has_location(&self) -> bool {
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_HasBaseField(self.raw_msg(), f)
        }
    }
    pub fn clear_location(&mut self) {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_ClearBaseField(
                self.raw_msg(),
                f,
            );
        }
    }
    pub fn location_opt(&self) -> ::protobuf::Optional<super::PointView<'_>> {
        ::protobuf::Optional::new(self.location(), self.has_location())
    }
    pub fn location(&self) -> super::PointView<'_> {
        let submsg = unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_GetMessage(self.raw_msg(), f)
        };
        match submsg {
            None => {
                super::PointView::new(
                    ::protobuf::__internal::Private,
                    ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
                )
            }
            Some(sub_raw_msg) => {
                super::PointView::new(::protobuf::__internal::Private, sub_raw_msg)
            }
        }
    }
    pub fn location_mut(&mut self) -> super::PointMut<'_> {
        let raw_msg = unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_GetOrCreateMutableMessage(
                    self.raw_msg(),
                    mt,
                    f,
                    self.arena().raw(),
                )
                .unwrap()
        };
        super::PointMut::from_parent(
            ::protobuf::__internal::Private,
            self.as_mutator_message_ref(::protobuf::__internal::Private),
            raw_msg,
        )
    }
    pub fn set_location(&mut self, val: impl ::protobuf::IntoProxied<super::Point>) {
        let mut msg = val.into_proxied(::protobuf::__internal::Private);
        self.as_mutator_message_ref(::protobuf::__internal::Private)
            .arena()
            .fuse(msg.as_mutator_message_ref(::protobuf::__internal::Private).arena());
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldMessage(
                self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
                f,
                msg.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
            );
        }
    }
    pub fn message(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
        let str_view = unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_GetString(
                self.raw_msg(),
                f,
                (b"").into(),
            )
        };
        unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
    }
    pub fn set_message(
        &mut self,
        val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>,
    ) {
        let s = val.into_proxied(::protobuf::__internal::Private);
        let (view, arena) = s
            .into_inner(::protobuf::__internal::Private)
            .into_raw_parts();
        let mm_ref = self.as_mutator_message_ref(::protobuf::__internal::Private);
        let parent_arena = mm_ref.arena();
        parent_arena.fuse(&arena);
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldString(
                self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
                f,
                view,
            );
        }
    }
}
unsafe impl Sync for RouteNoteMut<'_> {}
impl<'msg> ::protobuf::Proxy<'msg> for RouteNoteMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for RouteNoteMut<'msg> {}
impl<'msg> ::protobuf::AsView for RouteNoteMut<'msg> {
    type Proxied = RouteNote;
    fn as_view(&self) -> ::protobuf::View<'_, RouteNote> {
        RouteNoteView {
            msg: self.raw_msg(),
            _phantom: ::std::marker::PhantomData,
        }
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for RouteNoteMut<'msg> {
    fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RouteNote>
    where
        'msg: 'shorter,
    {
        RouteNoteView {
            msg: self.raw_msg(),
            _phantom: ::std::marker::PhantomData,
        }
    }
}
impl<'msg> ::protobuf::AsMut for RouteNoteMut<'msg> {
    type MutProxied = RouteNote;
    fn as_mut(&mut self) -> RouteNoteMut<'msg> {
        RouteNoteMut { inner: self.inner }
    }
}
impl<'msg> ::protobuf::IntoMut<'msg> for RouteNoteMut<'msg> {
    fn into_mut<'shorter>(self) -> RouteNoteMut<'shorter>
    where
        'msg: 'shorter,
    {
        self
    }
}
#[allow(dead_code)]
impl RouteNote {
    pub fn new() -> Self {
        let arena = ::protobuf::__internal::runtime::Arena::new();
        let raw_msg = unsafe {
            ::protobuf::__internal::runtime::upb_Message_New(
                    <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                    arena.raw(),
                )
                .unwrap()
        };
        Self {
            inner: ::protobuf::__internal::runtime::MessageInner {
                msg: raw_msg,
                arena,
            },
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.msg
    }
    #[doc(hidden)]
    pub fn as_mutator_message_ref(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MutatorMessageRef {
        ::protobuf::__internal::runtime::MutatorMessageRef::new(&mut self.inner)
    }
    fn arena(&self) -> &::protobuf::__internal::runtime::Arena {
        &self.inner.arena
    }
    pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        let mut msg = Self::new();
        ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
    }
    pub fn as_view(&self) -> RouteNoteView {
        RouteNoteView::new(::protobuf::__internal::Private, self.inner.msg)
    }
    pub fn as_mut(&mut self) -> RouteNoteMut {
        RouteNoteMut::new(::protobuf::__internal::Private, &mut self.inner)
    }
    pub fn has_location(&self) -> bool {
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_HasBaseField(self.raw_msg(), f)
        }
    }
    pub fn clear_location(&mut self) {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_ClearBaseField(
                self.raw_msg(),
                f,
            );
        }
    }
    pub fn location_opt(&self) -> ::protobuf::Optional<super::PointView<'_>> {
        ::protobuf::Optional::new(self.location(), self.has_location())
    }
    pub fn location(&self) -> super::PointView<'_> {
        let submsg = unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_GetMessage(self.raw_msg(), f)
        };
        match submsg {
            None => {
                super::PointView::new(
                    ::protobuf::__internal::Private,
                    ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
                )
            }
            Some(sub_raw_msg) => {
                super::PointView::new(::protobuf::__internal::Private, sub_raw_msg)
            }
        }
    }
    pub fn location_mut(&mut self) -> super::PointMut<'_> {
        let raw_msg = unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_GetOrCreateMutableMessage(
                    self.raw_msg(),
                    mt,
                    f,
                    self.arena().raw(),
                )
                .unwrap()
        };
        super::PointMut::from_parent(
            ::protobuf::__internal::Private,
            self.as_mutator_message_ref(::protobuf::__internal::Private),
            raw_msg,
        )
    }
    pub fn set_location(&mut self, val: impl ::protobuf::IntoProxied<super::Point>) {
        let mut msg = val.into_proxied(::protobuf::__internal::Private);
        self.as_mutator_message_ref(::protobuf::__internal::Private)
            .arena()
            .fuse(msg.as_mutator_message_ref(::protobuf::__internal::Private).arena());
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldMessage(
                self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
                f,
                msg.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
            );
        }
    }
    pub fn message(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
        let str_view = unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_GetString(
                self.raw_msg(),
                f,
                (b"").into(),
            )
        };
        unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
    }
    pub fn set_message(
        &mut self,
        val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>,
    ) {
        let s = val.into_proxied(::protobuf::__internal::Private);
        let (view, arena) = s
            .into_inner(::protobuf::__internal::Private)
            .into_raw_parts();
        let mm_ref = self.as_mutator_message_ref(::protobuf::__internal::Private);
        let parent_arena = mm_ref.arena();
        parent_arena.fuse(&arena);
        unsafe {
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldString(
                self.as_mutator_message_ref(::protobuf::__internal::Private).msg(),
                f,
                view,
            );
        }
    }
}
impl ::std::ops::Drop for RouteNote {
    fn drop(&mut self) {}
}
impl ::std::clone::Clone for RouteNote {
    fn clone(&self) -> Self {
        self.as_view().to_owned()
    }
}
impl ::protobuf::AsView for RouteNote {
    type Proxied = Self;
    fn as_view(&self) -> RouteNoteView {
        self.as_view()
    }
}
impl ::protobuf::AsMut for RouteNote {
    type MutProxied = Self;
    fn as_mut(&mut self) -> RouteNoteMut {
        self.as_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RouteNote {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        #[allow(unused_unsafe)]
        unsafe { ::std::ptr::addr_of!(routeguide__RouteNote_msg_init) }
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RouteNoteView<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        #[allow(unused_unsafe)]
        unsafe { ::std::ptr::addr_of!(routeguide__RouteNote_msg_init) }
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RouteNoteMut<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        #[allow(unused_unsafe)]
        unsafe { ::std::ptr::addr_of!(routeguide__RouteNote_msg_init) }
    }
}
extern "C" {
    /// Opaque static extern for this message's MiniTable, generated
    /// by the upb C MiniTable codegen. The only valid way to
    /// reference this static is with `std::ptr::addr_of!(..)`.
    static routeguide__RouteNote_msg_init: ::protobuf::__internal::runtime::upb_MiniTable;
}
impl ::protobuf::OwnedMessageInterop for RouteNote {}
impl<'a> ::protobuf::MessageMutInterop<'a> for RouteNoteMut<'a> {}
impl<'a> ::protobuf::MessageViewInterop<'a> for RouteNoteView<'a> {
    unsafe fn __unstable_wrap_raw_message(msg: &'a *const ::std::ffi::c_void) -> Self {
        Self::new(
            ::protobuf::__internal::Private,
            ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap(),
        )
    }
    unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
        msg: *const ::std::ffi::c_void,
    ) -> Self {
        Self::new(
            ::protobuf::__internal::Private,
            ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap(),
        )
    }
    fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
        self.msg.as_ptr() as *const _
    }
}
impl ::protobuf::__internal::MatcherEq for RouteNote {
    fn matches(&self, o: &Self) -> bool {
        ::protobuf::__internal::MatcherEq::matches(
            &::protobuf::AsView::as_view(self),
            &::protobuf::AsView::as_view(o),
        )
    }
}
impl<'a> ::protobuf::__internal::MatcherEq for RouteNoteMut<'a> {
    fn matches(&self, o: &Self) -> bool {
        ::protobuf::__internal::MatcherEq::matches(
            &::protobuf::AsView::as_view(self),
            &::protobuf::AsView::as_view(o),
        )
    }
}
impl<'a> ::protobuf::__internal::MatcherEq for RouteNoteView<'a> {
    fn matches(&self, o: &Self) -> bool {
        unsafe {
            ::protobuf::__internal::runtime::upb_Message_IsEqual(
                self.msg,
                o.msg,
                <RouteNote as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            )
        }
    }
}
#[allow(non_camel_case_types)]
pub struct RouteSummary {
    inner: ::protobuf::__internal::runtime::MessageInner,
}
impl ::protobuf::Message for RouteSummary {}
impl ::std::default::Default for RouteSummary {
    fn default() -> Self {
        Self::new()
    }
}
impl ::protobuf::Parse for RouteSummary {
    fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        Self::parse(serialized)
    }
}
impl ::std::fmt::Debug for RouteSummary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::TakeFrom for RouteSummary {
    fn take_from(&mut self, src: impl ::protobuf::AsMut<MutProxied = Self>) {
        let mut m = self.as_mut();
        ::protobuf::TakeFrom::take_from(&mut m, src)
    }
}
impl ::protobuf::CopyFrom for RouteSummary {
    fn copy_from(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
        let mut m = self.as_mut();
        ::protobuf::CopyFrom::copy_from(&mut m, src)
    }
}
impl ::protobuf::MergeFrom for RouteSummary {
    fn merge_from<'src>(&mut self, src: impl ::protobuf::AsView<Proxied = Self>) {
        let mut m = self.as_mut();
        ::protobuf::MergeFrom::merge_from(&mut m, src)
    }
}
impl ::protobuf::Serialize for RouteSummary {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        ::protobuf::AsView::as_view(self).serialize()
    }
}
impl ::protobuf::Clear for RouteSummary {
    fn clear(&mut self) {
        let mut m = self.as_mut();
        ::protobuf::Clear::clear(&mut m)
    }
}
impl ::protobuf::ClearAndParse for RouteSummary {
    fn clear_and_parse(
        &mut self,
        data: &[u8],
    ) -> ::std::result::Result<(), ::protobuf::ParseError> {
        let mut m = self.as_mut();
        ::protobuf::ClearAndParse::clear_and_parse(&mut m, data)
    }
}
unsafe impl Sync for RouteSummary {}
unsafe impl Send for RouteSummary {}
impl ::protobuf::Proxied for RouteSummary {
    type View<'msg> = RouteSummaryView<'msg>;
}
impl ::protobuf::__internal::SealedInternal for RouteSummary {}
impl ::protobuf::MutProxied for RouteSummary {
    type Mut<'msg> = RouteSummaryMut<'msg>;
}
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RouteSummaryView<'msg> {
    msg: ::protobuf::__internal::runtime::RawMessage,
    _phantom: ::std::marker::PhantomData<&'msg ()>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for RouteSummaryView<'msg> {}
impl<'msg> ::protobuf::MessageView<'msg> for RouteSummaryView<'msg> {
    type Message = RouteSummary;
}
impl ::std::fmt::Debug for RouteSummaryView<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for RouteSummaryView<'_> {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        let encoded = unsafe {
            ::protobuf::__internal::runtime::wire::encode(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        encoded.map_err(|_| ::protobuf::SerializeError)
    }
}
impl ::std::default::Default for RouteSummaryView<'_> {
    fn default() -> RouteSummaryView<'static> {
        RouteSummaryView::new(
            ::protobuf::__internal::Private,
            ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
        )
    }
}
#[allow(dead_code)]
impl<'msg> RouteSummaryView<'msg> {
    #[doc(hidden)]
    pub fn new(
        _private: ::protobuf::__internal::Private,
        msg: ::protobuf::__internal::runtime::RawMessage,
    ) -> Self {
        Self {
            msg,
            _phantom: ::std::marker::PhantomData,
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.msg
    }
    pub fn to_owned(&self) -> RouteSummary {
        ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
    }
    pub fn point_count(self) -> i32 {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_GetInt32(
                    self.raw_msg(),
                    f,
                    (0i32).into(),
                )
                .try_into()
                .unwrap()
        }
    }
    pub fn feature_count(self) -> i32 {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_GetInt32(
                    self.raw_msg(),
                    f,
                    (0i32).into(),
                )
                .try_into()
                .unwrap()
        }
    }
    pub fn distance(self) -> i32 {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                2,
            );
            ::protobuf::__internal::runtime::upb_Message_GetInt32(
                    self.raw_msg(),
                    f,
                    (0i32).into(),
                )
                .try_into()
                .unwrap()
        }
    }
    pub fn elapsed_time(self) -> i32 {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                3,
            );
            ::protobuf::__internal::runtime::upb_Message_GetInt32(
                    self.raw_msg(),
                    f,
                    (0i32).into(),
                )
                .try_into()
                .unwrap()
        }
    }
}
unsafe impl Sync for RouteSummaryView<'_> {}
unsafe impl Send for RouteSummaryView<'_> {}
impl<'msg> ::protobuf::Proxy<'msg> for RouteSummaryView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for RouteSummaryView<'msg> {}
impl<'msg> ::protobuf::AsView for RouteSummaryView<'msg> {
    type Proxied = RouteSummary;
    fn as_view(&self) -> ::protobuf::View<'msg, RouteSummary> {
        *self
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for RouteSummaryView<'msg> {
    fn into_view<'shorter>(self) -> RouteSummaryView<'shorter>
    where
        'msg: 'shorter,
    {
        self
    }
}
impl<'msg> ::protobuf::IntoProxied<RouteSummary> for RouteSummaryView<'msg> {
    fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RouteSummary {
        let dst = RouteSummary::new();
        unsafe {
            ::protobuf::__internal::runtime::upb_Message_DeepCopy(
                dst.inner.msg,
                self.msg,
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                dst.inner.arena.raw(),
            )
        };
        dst
    }
}
impl<'msg> ::protobuf::IntoProxied<RouteSummary> for RouteSummaryMut<'msg> {
    fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RouteSummary {
        ::protobuf::IntoProxied::into_proxied(
            ::protobuf::IntoView::into_view(self),
            _private,
        )
    }
}
unsafe impl ::protobuf::ProxiedInRepeated for RouteSummary {
    fn repeated_new(
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::Repeated<Self> {
        let arena = ::protobuf::__internal::runtime::Arena::new();
        unsafe {
            ::protobuf::Repeated::from_inner(
                ::protobuf::__internal::Private,
                ::protobuf::__internal::runtime::InnerRepeated::from_raw_parts(
                    ::protobuf::__internal::runtime::upb_Array_New(
                        arena.raw(),
                        ::protobuf::__internal::runtime::CType::Message,
                    ),
                    arena,
                ),
            )
        }
    }
    unsafe fn repeated_free(
        _private: ::protobuf::__internal::Private,
        _f: &mut ::protobuf::Repeated<Self>,
    ) {}
    fn repeated_len(f: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
        unsafe {
            ::protobuf::__internal::runtime::upb_Array_Size(
                f.as_raw(::protobuf::__internal::Private),
            )
        }
    }
    unsafe fn repeated_set_unchecked(
        mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
        i: usize,
        v: impl ::protobuf::IntoProxied<Self>,
    ) {
        unsafe {
            ::protobuf::__internal::runtime::upb_Array_Set(
                f.as_raw(::protobuf::__internal::Private),
                i,
                <Self as ::protobuf::__internal::runtime::UpbTypeConversions>::into_message_value_fuse_if_required(
                    f.raw_arena(::protobuf::__internal::Private),
                    v.into_proxied(::protobuf::__internal::Private),
                ),
            )
        }
    }
    unsafe fn repeated_get_unchecked(
        f: ::protobuf::View<::protobuf::Repeated<Self>>,
        i: usize,
    ) -> ::protobuf::View<Self> {
        let msg_ptr = unsafe {
            ::protobuf::__internal::runtime::upb_Array_Get(
                    f.as_raw(::protobuf::__internal::Private),
                    i,
                )
                .msg_val
        }
            .expect("upb_Array* element should not be NULL.");
        ::protobuf::View::<Self>::new(::protobuf::__internal::Private, msg_ptr)
    }
    unsafe fn repeated_get_mut_unchecked(
        mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
        i: usize,
    ) -> ::protobuf::Mut<Self> {
        let msg_ptr = unsafe {
            ::protobuf::__internal::runtime::upb_Array_GetMutable(
                f.as_raw(::protobuf::__internal::Private),
                i,
            )
        };
        unsafe {
            ::protobuf::Mut::<Self> {
                inner: ::protobuf::__internal::runtime::MutatorMessageRef::from_raw_parts(
                    msg_ptr,
                    f.arena(::protobuf::__internal::Private),
                ),
            }
        }
    }
    fn repeated_clear(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
        unsafe {
            ::protobuf::__internal::runtime::upb_Array_Resize(
                f.as_raw(::protobuf::__internal::Private),
                0,
                f.raw_arena(::protobuf::__internal::Private),
            )
        };
    }
    fn repeated_push(
        mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
        v: impl ::protobuf::IntoProxied<Self>,
    ) {
        unsafe {
            ::protobuf::__internal::runtime::upb_Array_Append(
                f.as_raw(::protobuf::__internal::Private),
                <Self as ::protobuf::__internal::runtime::UpbTypeConversions>::into_message_value_fuse_if_required(
                    f.raw_arena(::protobuf::__internal::Private),
                    v.into_proxied(::protobuf::__internal::Private),
                ),
                f.raw_arena(::protobuf::__internal::Private),
            );
        };
    }
    fn repeated_copy_from(
        src: ::protobuf::View<::protobuf::Repeated<Self>>,
        dest: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    ) {
        unsafe {
            ::protobuf::__internal::runtime::repeated_message_copy_from(
                src,
                dest,
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            );
        }
    }
    fn repeated_reserve(
        mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
        additional: usize,
    ) {
        unsafe {
            let size = ::protobuf::__internal::runtime::upb_Array_Size(
                f.as_raw(::protobuf::__internal::Private),
            );
            ::protobuf::__internal::runtime::upb_Array_Reserve(
                f.as_raw(::protobuf::__internal::Private),
                size + additional,
                f.raw_arena(::protobuf::__internal::Private),
            );
        }
    }
}
impl ::protobuf::__internal::runtime::UpbTypeConversions for RouteSummary {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }
    fn to_message_value(
        val: ::protobuf::View<'_, Self>,
    ) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue {
            msg_val: Some(val.raw_msg()),
        }
    }
    unsafe fn into_message_value_fuse_if_required(
        raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
        mut val: Self,
    ) -> ::protobuf::__internal::runtime::upb_MessageValue {
        let parent_arena = ::std::mem::ManuallyDrop::new(unsafe {
            ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena)
        });
        parent_arena
            .fuse(val.as_mutator_message_ref(::protobuf::__internal::Private).arena());
        ::protobuf::__internal::runtime::upb_MessageValue {
            msg_val: Some(val.raw_msg()),
        }
    }
    unsafe fn from_message_value<'msg>(
        msg: ::protobuf::__internal::runtime::upb_MessageValue,
    ) -> ::protobuf::View<'msg, Self> {
        RouteSummaryView::new(
            ::protobuf::__internal::Private,
            unsafe { msg.msg_val }.expect("expected present message value in map"),
        )
    }
    unsafe fn from_message_mut<'msg>(
        msg: *mut ::protobuf::__internal::runtime::upb_Message,
        arena: &'msg ::protobuf::__internal::runtime::Arena,
    ) -> RouteSummaryMut<'msg> {
        RouteSummaryMut {
            inner: unsafe {
                ::protobuf::__internal::runtime::MutatorMessageRef::from_raw_parts(
                    std::ptr::NonNull::new(msg)
                        .expect("expected present message value in map"),
                    arena,
                )
            },
        }
    }
}
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RouteSummaryMut<'msg> {
    inner: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for RouteSummaryMut<'msg> {}
impl<'msg> ::protobuf::MessageMut<'msg> for RouteSummaryMut<'msg> {
    type Message = RouteSummary;
}
impl ::std::fmt::Debug for RouteSummaryMut<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for RouteSummaryMut<'_> {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        ::protobuf::AsView::as_view(self).serialize()
    }
}
impl ::protobuf::Clear for RouteSummaryMut<'_> {
    fn clear(&mut self) {
        unsafe {
            ::protobuf::__internal::runtime::upb_Message_Clear(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        }
    }
}
impl ::protobuf::ClearAndParse for RouteSummaryMut<'_> {
    fn clear_and_parse(
        &mut self,
        data: &[u8],
    ) -> ::std::result::Result<(), ::protobuf::ParseError> {
        ::protobuf::Clear::clear(self);
        let status = unsafe {
            ::protobuf::__internal::runtime::wire::decode(
                data,
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                self.arena(),
            )
        };
        match status {
            Ok(_) => Ok(()),
            Err(_) => Err(::protobuf::ParseError),
        }
    }
}
impl ::protobuf::TakeFrom for RouteSummaryMut<'_> {
    fn take_from(&mut self, mut src: impl ::protobuf::AsMut<MutProxied = RouteSummary>) {
        let mut src = src.as_mut();
        ::protobuf::CopyFrom::copy_from(self, ::protobuf::AsView::as_view(&src));
        ::protobuf::Clear::clear(&mut src);
    }
}
impl ::protobuf::CopyFrom for RouteSummaryMut<'_> {
    fn copy_from(&mut self, src: impl ::protobuf::AsView<Proxied = RouteSummary>) {
        unsafe {
            assert!(
                ::protobuf::__internal::runtime::upb_Message_DeepCopy(self.raw_msg(), src
                .as_view().raw_msg(), < Self as
                ::protobuf::__internal::runtime::AssociatedMiniTable >::mini_table(),
                self.arena().raw())
            );
        }
    }
}
impl ::protobuf::MergeFrom for RouteSummaryMut<'_> {
    fn merge_from(&mut self, src: impl ::protobuf::AsView<Proxied = RouteSummary>) {
        unsafe {
            assert!(
                ::protobuf::__internal::runtime::upb_Message_MergeFrom(self.raw_msg(),
                src.as_view().raw_msg(), < Self as
                ::protobuf::__internal::runtime::AssociatedMiniTable >::mini_table(),
                ::std::ptr::null(), self.arena().raw())
            );
        }
    }
}
#[allow(dead_code)]
impl<'msg> RouteSummaryMut<'msg> {
    #[doc(hidden)]
    pub fn from_parent(
        _private: ::protobuf::__internal::Private,
        parent: ::protobuf::__internal::runtime::MutatorMessageRef<'msg>,
        msg: ::protobuf::__internal::runtime::RawMessage,
    ) -> Self {
        Self {
            inner: ::protobuf::__internal::runtime::MutatorMessageRef::from_parent(
                parent,
                msg,
            ),
        }
    }
    #[doc(hidden)]
    pub fn new(
        _private: ::protobuf::__internal::Private,
        msg: &'msg mut ::protobuf::__internal::runtime::MessageInner,
    ) -> Self {
        Self {
            inner: ::protobuf::__internal::runtime::MutatorMessageRef::new(msg),
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.msg()
    }
    #[doc(hidden)]
    pub fn as_mutator_message_ref(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MutatorMessageRef<'msg> {
        self.inner
    }
    pub fn to_owned(&self) -> RouteSummary {
        ::protobuf::AsView::as_view(self).to_owned()
    }
    fn arena(&self) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
    pub fn point_count(&self) -> i32 {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_GetInt32(
                    self.raw_msg(),
                    f,
                    (0i32).into(),
                )
                .try_into()
                .unwrap()
        }
    }
    pub fn set_point_count(&mut self, val: i32) {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldInt32(
                self.raw_msg(),
                f,
                val.into(),
            );
        }
    }
    pub fn feature_count(&self) -> i32 {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_GetInt32(
                    self.raw_msg(),
                    f,
                    (0i32).into(),
                )
                .try_into()
                .unwrap()
        }
    }
    pub fn set_feature_count(&mut self, val: i32) {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldInt32(
                self.raw_msg(),
                f,
                val.into(),
            );
        }
    }
    pub fn distance(&self) -> i32 {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                2,
            );
            ::protobuf::__internal::runtime::upb_Message_GetInt32(
                    self.raw_msg(),
                    f,
                    (0i32).into(),
                )
                .try_into()
                .unwrap()
        }
    }
    pub fn set_distance(&mut self, val: i32) {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                2,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldInt32(
                self.raw_msg(),
                f,
                val.into(),
            );
        }
    }
    pub fn elapsed_time(&self) -> i32 {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                3,
            );
            ::protobuf::__internal::runtime::upb_Message_GetInt32(
                    self.raw_msg(),
                    f,
                    (0i32).into(),
                )
                .try_into()
                .unwrap()
        }
    }
    pub fn set_elapsed_time(&mut self, val: i32) {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                3,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldInt32(
                self.raw_msg(),
                f,
                val.into(),
            );
        }
    }
}
unsafe impl Sync for RouteSummaryMut<'_> {}
impl<'msg> ::protobuf::Proxy<'msg> for RouteSummaryMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for RouteSummaryMut<'msg> {}
impl<'msg> ::protobuf::AsView for RouteSummaryMut<'msg> {
    type Proxied = RouteSummary;
    fn as_view(&self) -> ::protobuf::View<'_, RouteSummary> {
        RouteSummaryView {
            msg: self.raw_msg(),
            _phantom: ::std::marker::PhantomData,
        }
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for RouteSummaryMut<'msg> {
    fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RouteSummary>
    where
        'msg: 'shorter,
    {
        RouteSummaryView {
            msg: self.raw_msg(),
            _phantom: ::std::marker::PhantomData,
        }
    }
}
impl<'msg> ::protobuf::AsMut for RouteSummaryMut<'msg> {
    type MutProxied = RouteSummary;
    fn as_mut(&mut self) -> RouteSummaryMut<'msg> {
        RouteSummaryMut {
            inner: self.inner,
        }
    }
}
impl<'msg> ::protobuf::IntoMut<'msg> for RouteSummaryMut<'msg> {
    fn into_mut<'shorter>(self) -> RouteSummaryMut<'shorter>
    where
        'msg: 'shorter,
    {
        self
    }
}
#[allow(dead_code)]
impl RouteSummary {
    pub fn new() -> Self {
        let arena = ::protobuf::__internal::runtime::Arena::new();
        let raw_msg = unsafe {
            ::protobuf::__internal::runtime::upb_Message_New(
                    <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                    arena.raw(),
                )
                .unwrap()
        };
        Self {
            inner: ::protobuf::__internal::runtime::MessageInner {
                msg: raw_msg,
                arena,
            },
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.msg
    }
    #[doc(hidden)]
    pub fn as_mutator_message_ref(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MutatorMessageRef {
        ::protobuf::__internal::runtime::MutatorMessageRef::new(&mut self.inner)
    }
    fn arena(&self) -> &::protobuf::__internal::runtime::Arena {
        &self.inner.arena
    }
    pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        let mut msg = Self::new();
        ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
    }
    pub fn as_view(&self) -> RouteSummaryView {
        RouteSummaryView::new(::protobuf::__internal::Private, self.inner.msg)
    }
    pub fn as_mut(&mut self) -> RouteSummaryMut {
        RouteSummaryMut::new(::protobuf::__internal::Private, &mut self.inner)
    }
    pub fn point_count(&self) -> i32 {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_GetInt32(
                    self.raw_msg(),
                    f,
                    (0i32).into(),
                )
                .try_into()
                .unwrap()
        }
    }
    pub fn set_point_count(&mut self, val: i32) {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                0,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldInt32(
                self.raw_msg(),
                f,
                val.into(),
            );
        }
    }
    pub fn feature_count(&self) -> i32 {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_GetInt32(
                    self.raw_msg(),
                    f,
                    (0i32).into(),
                )
                .try_into()
                .unwrap()
        }
    }
    pub fn set_feature_count(&mut self, val: i32) {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                1,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldInt32(
                self.raw_msg(),
                f,
                val.into(),
            );
        }
    }
    pub fn distance(&self) -> i32 {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                2,
            );
            ::protobuf::__internal::runtime::upb_Message_GetInt32(
                    self.raw_msg(),
                    f,
                    (0i32).into(),
                )
                .try_into()
                .unwrap()
        }
    }
    pub fn set_distance(&mut self, val: i32) {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                2,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldInt32(
                self.raw_msg(),
                f,
                val.into(),
            );
        }
    }
    pub fn elapsed_time(&self) -> i32 {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                3,
            );
            ::protobuf::__internal::runtime::upb_Message_GetInt32(
                    self.raw_msg(),
                    f,
                    (0i32).into(),
                )
                .try_into()
                .unwrap()
        }
    }
    pub fn set_elapsed_time(&mut self, val: i32) {
        unsafe {
            let mt = <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
            let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
                mt,
                3,
            );
            ::protobuf::__internal::runtime::upb_Message_SetBaseFieldInt32(
                self.raw_msg(),
                f,
                val.into(),
            );
        }
    }
}
impl ::std::ops::Drop for RouteSummary {
    fn drop(&mut self) {}
}
impl ::std::clone::Clone for RouteSummary {
    fn clone(&self) -> Self {
        self.as_view().to_owned()
    }
}
impl ::protobuf::AsView for RouteSummary {
    type Proxied = Self;
    fn as_view(&self) -> RouteSummaryView {
        self.as_view()
    }
}
impl ::protobuf::AsMut for RouteSummary {
    type MutProxied = Self;
    fn as_mut(&mut self) -> RouteSummaryMut {
        self.as_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RouteSummary {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        #[allow(unused_unsafe)]
        unsafe { ::std::ptr::addr_of!(routeguide__RouteSummary_msg_init) }
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable
for RouteSummaryView<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        #[allow(unused_unsafe)]
        unsafe { ::std::ptr::addr_of!(routeguide__RouteSummary_msg_init) }
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable
for RouteSummaryMut<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        #[allow(unused_unsafe)]
        unsafe { ::std::ptr::addr_of!(routeguide__RouteSummary_msg_init) }
    }
}
extern "C" {
    /// Opaque static extern for this message's MiniTable, generated
    /// by the upb C MiniTable codegen. The only valid way to
    /// reference this static is with `std::ptr::addr_of!(..)`.
    static routeguide__RouteSummary_msg_init: ::protobuf::__internal::runtime::upb_MiniTable;
}
impl ::protobuf::OwnedMessageInterop for RouteSummary {}
impl<'a> ::protobuf::MessageMutInterop<'a> for RouteSummaryMut<'a> {}
impl<'a> ::protobuf::MessageViewInterop<'a> for RouteSummaryView<'a> {
    unsafe fn __unstable_wrap_raw_message(msg: &'a *const ::std::ffi::c_void) -> Self {
        Self::new(
            ::protobuf::__internal::Private,
            ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap(),
        )
    }
    unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
        msg: *const ::std::ffi::c_void,
    ) -> Self {
        Self::new(
            ::protobuf::__internal::Private,
            ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap(),
        )
    }
    fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
        self.msg.as_ptr() as *const _
    }
}
impl ::protobuf::__internal::MatcherEq for RouteSummary {
    fn matches(&self, o: &Self) -> bool {
        ::protobuf::__internal::MatcherEq::matches(
            &::protobuf::AsView::as_view(self),
            &::protobuf::AsView::as_view(o),
        )
    }
}
impl<'a> ::protobuf::__internal::MatcherEq for RouteSummaryMut<'a> {
    fn matches(&self, o: &Self) -> bool {
        ::protobuf::__internal::MatcherEq::matches(
            &::protobuf::AsView::as_view(self),
            &::protobuf::AsView::as_view(o),
        )
    }
}
impl<'a> ::protobuf::__internal::MatcherEq for RouteSummaryView<'a> {
    fn matches(&self, o: &Self) -> bool {
        unsafe {
            ::protobuf::__internal::runtime::upb_Message_IsEqual(
                self.msg,
                o.msg,
                <RouteSummary as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                0,
            )
        }
    }
}
