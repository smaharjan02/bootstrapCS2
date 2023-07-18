select count(*) from lineitem ll where L_QUANTITY < 20 and L_QUANTITY > 0
select count(*) from lineitem ll where L_LINENUMBER < 3 and L_LINENUMBER > 0
select count(*) from lineitem ll where L_LINENUMBER < 5 and L_LINENUMBER > 2
select count(*) from lineitem ll where L_DISCOUNT < .07 and L_DISCOUNT > .02
select count(*) from lineitem ll where L_EXTENDEDPRICE < 100000.00 and L_EXTENDEDPRICE > 20000.00
select count(*) from lineitem ll where L_DISCOUNT < .04 and L_DISCOUNT> 0.0
select count(*) from lineitem ll where L_QUANTITY < 20 and L_QUANTITY > 10
select count(*) from lineitem ll where L_DISCOUNT < .05 and L_DISCOUNT > .02
select count(*) from lineitem ll where L_EXTENDEDPRICE < 15000.00 and L_EXTENDEDPRICE > 0.0
select count(*) from lineitem ll where L_LINENUMBER < 2 and L_LINENUMBER > 0