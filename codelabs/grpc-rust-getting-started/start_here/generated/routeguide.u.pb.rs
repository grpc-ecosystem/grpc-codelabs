const _: () = ::protobuf::__internal::assert_compatible_gencode_version(
    "4.35.1-release",
);
pub(crate) static mut routeguide__Point_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr = ::protobuf::__internal::runtime::MiniTableInitPtr(
    ::protobuf::__internal::runtime::MiniTablePtr::dangling(),
);
#[allow(non_camel_case_types)]
pub struct Point {
    inner: ::protobuf::__internal::runtime::OwnedMessageInner<Point>,
}
impl ::protobuf::Message for Point {
    type MessageView<'msg> = PointView<'msg>;
    type MessageMut<'msg> = PointMut<'msg>;
}
impl ::std::default::Default for Point {
    fn default() -> Self {
        Self::new()
    }
}
impl ::std::fmt::Debug for Point {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
    }
}
unsafe impl ::std::marker::Sync for Point {}
unsafe impl ::std::marker::Send for Point {}
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
    inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Point>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for PointView<'msg> {}
impl<'msg> ::protobuf::MessageView<'msg> for PointView<'msg> {
    type Message = Point;
}
impl ::std::fmt::Debug for PointView<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
    }
}
impl ::std::default::Default for PointView<'_> {
    fn default() -> PointView<'static> {
        ::protobuf::__internal::runtime::MessageViewInner::default().into()
    }
}
impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Point>>
for PointView<'msg> {
    fn from(
        inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Point>,
    ) -> Self {
        Self { inner }
    }
}
#[allow(dead_code)]
impl<'msg> PointView<'msg> {
    pub fn to_owned(&self) -> Point {
        ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
    }
}
unsafe impl ::std::marker::Sync for PointView<'_> {}
unsafe impl ::std::marker::Send for PointView<'_> {}
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
        let mut dst = Point::new();
        assert!(
            unsafe { dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena()) }
        );
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
impl ::protobuf::__internal::EntityType for Point {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}
impl<'msg> ::protobuf::__internal::EntityType for PointView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}
impl<'msg> ::protobuf::__internal::EntityType for PointMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PointMut<'msg> {
    inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Point>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for PointMut<'msg> {}
impl<'msg> ::protobuf::MessageMut<'msg> for PointMut<'msg> {
    type Message = Point;
}
impl ::std::fmt::Debug for PointMut<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
    }
}
impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Point>>
for PointMut<'msg> {
    fn from(
        inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Point>,
    ) -> Self {
        Self { inner }
    }
}
#[allow(dead_code)]
impl<'msg> PointMut<'msg> {
    #[doc(hidden)]
    pub fn as_message_mut_inner(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Point> {
        self.inner.reborrow()
    }
    pub fn to_owned(&self) -> Point {
        ::protobuf::AsView::as_view(self).to_owned()
    }
}
unsafe impl ::std::marker::Send for PointMut<'_> {}
unsafe impl ::std::marker::Sync for PointMut<'_> {}
impl<'msg> ::protobuf::AsView for PointMut<'msg> {
    type Proxied = Point;
    fn as_view(&self) -> ::protobuf::View<'_, Point> {
        self.inner.as_view().into()
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for PointMut<'msg> {
    fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Point>
    where
        'msg: 'shorter,
    {
        self.inner.as_view().into()
    }
}
impl<'msg> ::protobuf::AsMut for PointMut<'msg> {
    type MutProxied = Point;
    fn as_mut(&mut self) -> PointMut<'msg> {
        self.inner.reborrow().into()
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
        Self {
            inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new(),
        }
    }
    #[doc(hidden)]
    pub fn as_message_mut_inner(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Point> {
        ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
    }
    pub fn as_view(&self) -> PointView<'_> {
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner)
            .into()
    }
    pub fn as_mut(&mut self) -> PointMut<'_> {
        ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
            .into()
    }
}
impl ::std::ops::Drop for Point {
    #[inline]
    fn drop(&mut self) {}
}
impl ::std::clone::Clone for Point {
    fn clone(&self) -> Self {
        self.as_view().to_owned()
    }
}
impl ::protobuf::AsView for Point {
    type Proxied = Self;
    fn as_view(&self) -> PointView<'_> {
        self.as_view()
    }
}
impl ::protobuf::AsMut for Point {
    type MutProxied = Self;
    fn as_mut(&mut self) -> PointMut<'_> {
        self.as_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Point {
    fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
        static ONCE_LOCK: ::std::sync::OnceLock<
            ::protobuf::__internal::runtime::MiniTableInitPtr,
        > = ::std::sync::OnceLock::new();
        unsafe {
            ONCE_LOCK
                .get_or_init(|| {
                    super::routeguide__Point_msg_init.0 = ::protobuf::__internal::runtime::build_mini_table(
                        "$",
                    );
                    ::protobuf::__internal::runtime::link_mini_table(
                        super::routeguide__Point_msg_init.0,
                        &[],
                        &[],
                    );
                    ::protobuf::__internal::runtime::MiniTableInitPtr(
                        super::routeguide__Point_msg_init.0,
                    )
                })
                .0
        }
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Point {
    fn get_arena(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Point {
    type Msg = Point;
    fn get_ptr_mut(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Point> {
        self.inner.ptr_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Point {
    type Msg = Point;
    fn get_ptr(
        &self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Point> {
        self.inner.ptr()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PointMut<'_> {
    type Msg = Point;
    fn get_ptr_mut(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Point> {
        self.inner.ptr_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PointMut<'_> {
    type Msg = Point;
    fn get_ptr(
        &self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Point> {
        self.inner.ptr()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PointView<'_> {
    type Msg = Point;
    fn get_ptr(
        &self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Point> {
        self.inner.ptr()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PointMut<'_> {
    fn get_arena(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
}
pub(crate) static mut routeguide__Feature_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr = ::protobuf::__internal::runtime::MiniTableInitPtr(
    ::protobuf::__internal::runtime::MiniTablePtr::dangling(),
);
#[allow(non_camel_case_types)]
pub struct Feature {
    inner: ::protobuf::__internal::runtime::OwnedMessageInner<Feature>,
}
impl ::protobuf::Message for Feature {
    type MessageView<'msg> = FeatureView<'msg>;
    type MessageMut<'msg> = FeatureMut<'msg>;
}
impl ::std::default::Default for Feature {
    fn default() -> Self {
        Self::new()
    }
}
impl ::std::fmt::Debug for Feature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
    }
}
unsafe impl ::std::marker::Sync for Feature {}
unsafe impl ::std::marker::Send for Feature {}
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
    inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Feature>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for FeatureView<'msg> {}
impl<'msg> ::protobuf::MessageView<'msg> for FeatureView<'msg> {
    type Message = Feature;
}
impl ::std::fmt::Debug for FeatureView<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
    }
}
impl ::std::default::Default for FeatureView<'_> {
    fn default() -> FeatureView<'static> {
        ::protobuf::__internal::runtime::MessageViewInner::default().into()
    }
}
impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Feature>>
for FeatureView<'msg> {
    fn from(
        inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Feature>,
    ) -> Self {
        Self { inner }
    }
}
#[allow(dead_code)]
impl<'msg> FeatureView<'msg> {
    pub fn to_owned(&self) -> Feature {
        ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
    }
}
unsafe impl ::std::marker::Sync for FeatureView<'_> {}
unsafe impl ::std::marker::Send for FeatureView<'_> {}
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
        let mut dst = Feature::new();
        assert!(
            unsafe { dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena()) }
        );
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
impl ::protobuf::__internal::EntityType for Feature {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}
impl<'msg> ::protobuf::__internal::EntityType for FeatureView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}
impl<'msg> ::protobuf::__internal::EntityType for FeatureMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FeatureMut<'msg> {
    inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Feature>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for FeatureMut<'msg> {}
impl<'msg> ::protobuf::MessageMut<'msg> for FeatureMut<'msg> {
    type Message = Feature;
}
impl ::std::fmt::Debug for FeatureMut<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
    }
}
impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Feature>>
for FeatureMut<'msg> {
    fn from(
        inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Feature>,
    ) -> Self {
        Self { inner }
    }
}
#[allow(dead_code)]
impl<'msg> FeatureMut<'msg> {
    #[doc(hidden)]
    pub fn as_message_mut_inner(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Feature> {
        self.inner.reborrow()
    }
    pub fn to_owned(&self) -> Feature {
        ::protobuf::AsView::as_view(self).to_owned()
    }
}
unsafe impl ::std::marker::Send for FeatureMut<'_> {}
unsafe impl ::std::marker::Sync for FeatureMut<'_> {}
impl<'msg> ::protobuf::AsView for FeatureMut<'msg> {
    type Proxied = Feature;
    fn as_view(&self) -> ::protobuf::View<'_, Feature> {
        self.inner.as_view().into()
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for FeatureMut<'msg> {
    fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Feature>
    where
        'msg: 'shorter,
    {
        self.inner.as_view().into()
    }
}
impl<'msg> ::protobuf::AsMut for FeatureMut<'msg> {
    type MutProxied = Feature;
    fn as_mut(&mut self) -> FeatureMut<'msg> {
        self.inner.reborrow().into()
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
        Self {
            inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new(),
        }
    }
    #[doc(hidden)]
    pub fn as_message_mut_inner(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Feature> {
        ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
    }
    pub fn as_view(&self) -> FeatureView<'_> {
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner)
            .into()
    }
    pub fn as_mut(&mut self) -> FeatureMut<'_> {
        ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
            .into()
    }
}
impl ::std::ops::Drop for Feature {
    #[inline]
    fn drop(&mut self) {}
}
impl ::std::clone::Clone for Feature {
    fn clone(&self) -> Self {
        self.as_view().to_owned()
    }
}
impl ::protobuf::AsView for Feature {
    type Proxied = Self;
    fn as_view(&self) -> FeatureView<'_> {
        self.as_view()
    }
}
impl ::protobuf::AsMut for Feature {
    type MutProxied = Self;
    fn as_mut(&mut self) -> FeatureMut<'_> {
        self.as_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Feature {
    fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
        static ONCE_LOCK: ::std::sync::OnceLock<
            ::protobuf::__internal::runtime::MiniTableInitPtr,
        > = ::std::sync::OnceLock::new();
        unsafe {
            ONCE_LOCK
                .get_or_init(|| {
                    super::routeguide__Feature_msg_init.0 = ::protobuf::__internal::runtime::build_mini_table(
                        "$",
                    );
                    ::protobuf::__internal::runtime::link_mini_table(
                        super::routeguide__Feature_msg_init.0,
                        &[],
                        &[],
                    );
                    ::protobuf::__internal::runtime::MiniTableInitPtr(
                        super::routeguide__Feature_msg_init.0,
                    )
                })
                .0
        }
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Feature {
    fn get_arena(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Feature {
    type Msg = Feature;
    fn get_ptr_mut(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Feature> {
        self.inner.ptr_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Feature {
    type Msg = Feature;
    fn get_ptr(
        &self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Feature> {
        self.inner.ptr()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FeatureMut<'_> {
    type Msg = Feature;
    fn get_ptr_mut(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Feature> {
        self.inner.ptr_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FeatureMut<'_> {
    type Msg = Feature;
    fn get_ptr(
        &self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Feature> {
        self.inner.ptr()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FeatureView<'_> {
    type Msg = Feature;
    fn get_ptr(
        &self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Feature> {
        self.inner.ptr()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FeatureMut<'_> {
    fn get_arena(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
}
