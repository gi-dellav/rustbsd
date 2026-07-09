#	$OpenBSD$

PROG=	cat
CLEANFILES+=	target

${PROG}:
	cargo build --release
	ln -sf target/release/${PROG} ${PROG}

.include <bsd.prog.mk>
