/// Generated client implementations.
pub mod route_guide_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        unused_imports,
    )]
    use grpc::client::*;
    use grpc_protobuf::*;

    /// Interface exported by the server.
    #[derive(Debug, Clone)]
    pub struct RouteGuideClient<T> {
        channel: T,
    }

    impl<T> RouteGuideClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        /// A server-to-client streaming RPC.
        ///
        /// Obtains the Features available within the given Rectangle.  Results are
        /// streamed rather than returned at once (e.g. in a response message with a
        /// repeated field), as the rectangle may cover a large area and contain a
        /// huge number of features.
        pub fn list_features<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> ServerStreamingCallBuilder<'_, &T, ReqMsgView, super::Feature>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::Rectangle> + Send + Sync {
          ServerStreamingCallBuilder::new(&self.channel, "/routeguide.RouteGuide/ListFeatures", request)
        }

        /// A client-to-server streaming RPC.
        ///
        /// Accepts a stream of Points on a route being traversed, returning a
        /// RouteSummary when traversal is completed.
        pub fn record_route(&self) -> ClientStreamingCallBuilder<'_, &T, super::Point, super::RouteSummary> {
          ClientStreamingCallBuilder::new(&self.channel, "/routeguide.RouteGuide/RecordRoute")
        }

        /// A Bidirectional streaming RPC.
        ///
        /// Accepts a stream of RouteNotes sent while a route is being traversed,
        /// while receiving other RouteNotes (e.g. from other users).
        pub fn route_chat(&self) -> BidiCallBuilder<'_, &T, super::RouteNote, super::RouteNote> {
          BidiCallBuilder::new(&self.channel, "/routeguide.RouteGuide/RouteChat")
        }
    }
}

/// Generated server implementations.
pub mod route_guide_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        // will trigger if compression is disabled
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;

    /// Generated trait containing gRPC methods that should be implemented for use with RouteGuideServer.

    #[async_trait]
    pub trait RouteGuide : std::marker::Send + std::marker::Sync + 'static {
        /// A server-to-client streaming RPC.
///
/// Obtains the Features available within the given Rectangle.  Results are
/// streamed rather than returned at once (e.g. in a response message with a
/// repeated field), as the rectangle may cover a large area and contain a
/// huge number of features.

        async fn list_features(&self, request: tonic::Request<super::Rectangle>)
            -> std::result::Result<tonic::Response<BoxStream<super::Feature>>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        /// A client-to-server streaming RPC.
///
/// Accepts a stream of Points on a route being traversed, returning a
/// RouteSummary when traversal is completed.

        async fn record_route(&self, request: tonic::Request<tonic::Streaming<super::Point>>)
            -> std::result::Result<tonic::Response<super::RouteSummary>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }

        /// A Bidirectional streaming RPC.
///
/// Accepts a stream of RouteNotes sent while a route is being traversed,
/// while receiving other RouteNotes (e.g. from other users).

        async fn route_chat(&self, request: tonic::Request<tonic::Streaming<super::RouteNote>>)
            -> std::result::Result<tonic::Response<BoxStream<super::RouteNote>>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
    }

    /// Interface exported by the server.

    #[derive(Debug)]
    pub struct RouteGuideServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }

    impl<T> RouteGuideServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }

        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }

        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }

        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }

        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }

        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }

        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }

    impl<T, B> tonic::codegen::Service<http::Request<B>> for RouteGuideServer<T>
        where
            T: RouteGuide,
            B: Body + std::marker::Send + 'static,
            B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/routeguide.RouteGuide/ListFeatures" => {
                    #[allow(non_camel_case_types)]
                    struct list_featuresSvc<T: RouteGuide >(pub Arc<T>);

                    impl<T: RouteGuide> tonic::server::ServerStreamingService<super::Rectangle> for list_featuresSvc<T> {
                        type Response = super::Feature;
                        type ResponseStream = BoxStream<super::Feature>;
                        type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<super::Rectangle>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RouteGuide>::list_features(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = list_featuresSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/routeguide.RouteGuide/RecordRoute" => {
                    #[allow(non_camel_case_types)]
                    struct record_routeSvc<T: RouteGuide >(pub Arc<T>);

                    impl<T: RouteGuide> tonic::server::ClientStreamingService<super::Point> for record_routeSvc<T>
                    {
                        type Response = super::RouteSummary;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<tonic::Streaming<super::Point>>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RouteGuide>::record_route(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = record_routeSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }
                "/routeguide.RouteGuide/RouteChat" => {
                    #[allow(non_camel_case_types)]
                    struct route_chatSvc<T: RouteGuide>(pub Arc<T>);

                    impl<T: RouteGuide> tonic::server::StreamingService<super::RouteNote> for route_chatSvc<T>
                    {
                        type Response = super::RouteNote;
                        type ResponseStream = BoxStream<super::RouteNote>;
                        type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;

                        fn call(&mut self, request: tonic::Request<tonic::Streaming<super::RouteNote>>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RouteGuide>::route_chat(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }

                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = route_chatSvc(inner);
                        let codec = tonic_protobuf::ProtoCodec::default();

                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept_compression_encodings, send_compression_encodings)
                            .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);

                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };

                    Box::pin(fut)
                }

                _ => Box::pin(async move {
                    let mut response = http::Response::new(tonic::body::Body::default());
                    let headers = response.headers_mut();
                    headers.insert(tonic::Status::GRPC_STATUS, (tonic::Code::Unimplemented as i32).into());
                    headers.insert(http::header::CONTENT_TYPE, tonic::metadata::GRPC_CONTENT_TYPE);
                    Ok(response)
                }),
            }
        }
    }

    impl<T> Clone for RouteGuideServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }

    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "routeguide.RouteGuide";

    impl<T> tonic::server::NamedService for RouteGuideServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
