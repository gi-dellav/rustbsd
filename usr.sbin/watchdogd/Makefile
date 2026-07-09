#	$OpenBSD$

PROG=	watchdogd
CLEANFILES+=	target
MAN=	watchdogd.8

${PROG}:
	cargo build --release --manifest-path ${.CURDIR}/Cargo.toml
	ln -sf target/release/${PROG} ${PROG}

.include <bsd.prog.mk>
