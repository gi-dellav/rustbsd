#	$OpenBSD$

PROG=	watchdogd
CLEANFILES+=	target
MAN=	watchdogd.8

${PROG}:
	cargo build --release
	ln -sf target/release/${PROG} ${PROG}

.include <bsd.prog.mk>
