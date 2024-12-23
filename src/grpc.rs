pub mod chainrpc {
    tonic::include_proto!("chainrpc");
}

pub mod invoicesrpc {
    tonic::include_proto!("invoicesrpc");
}

#[expect(clippy::style, reason = "codegen")]
pub mod lnrpc {
    tonic::include_proto!("lnrpc");
}

pub mod routerrpc {
    tonic::include_proto!("routerrpc");
}
