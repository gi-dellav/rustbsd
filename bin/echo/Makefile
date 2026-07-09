#	$OpenBSD$

PROG=	echo
CLEANFILES+=	target

${PROG}:
	cargo build --release --manifest-path ${.CURDIR}/Cargo.toml
	ln -sf target/release/${PROG} ${PROG}

.include <bsd.prog.mk>
