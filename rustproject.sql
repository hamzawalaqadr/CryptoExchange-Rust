PGDMP     )    1                {            rustsample2    11.19    11.19 F    `           0    0    ENCODING    ENCODING        SET client_encoding = 'UTF8';
                       false            a           0    0 
   STDSTRINGS 
   STDSTRINGS     (   SET standard_conforming_strings = 'on';
                       false            b           0    0 
   SEARCHPATH 
   SEARCHPATH     8   SELECT pg_catalog.set_config('search_path', '', false);
                       false            c           1262    16393    rustsample2    DATABASE     �   CREATE DATABASE rustsample2 WITH TEMPLATE = template0 ENCODING = 'UTF8' LC_COLLATE = 'Sindhi_Pakistan.1256' LC_CTYPE = 'Sindhi_Pakistan.1256';
    DROP DATABASE rustsample2;
             postgres    false            �            1255    33563 "   diesel_manage_updated_at(regclass)    FUNCTION       CREATE FUNCTION public.diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;
 >   DROP FUNCTION public.diesel_manage_updated_at(_tbl regclass);
       public       postgres    false            �            1255    33564    diesel_set_updated_at()    FUNCTION     *  CREATE FUNCTION public.diesel_set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;
 .   DROP FUNCTION public.diesel_set_updated_at();
       public       postgres    false            �            1255    24740    update_modified_on_column()    FUNCTION     �   CREATE FUNCTION public.update_modified_on_column() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
   NEW.modified_on = CURRENT_TIMESTAMP;
   RETURN NEW;
END;
$$;
 2   DROP FUNCTION public.update_modified_on_column();
       public       postgres    false            �            1259    16394    __diesel_schema_migrations    TABLE     �   CREATE TABLE public.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);
 .   DROP TABLE public.__diesel_schema_migrations;
       public         postgres    false            �            1259    33585    crypto    TABLE       CREATE TABLE public.crypto (
    id integer NOT NULL,
    cname character varying(255) NOT NULL,
    symbol character varying(255) NOT NULL,
    price double precision NOT NULL,
    created_on timestamp with time zone,
    modified_on timestamp with time zone
);
    DROP TABLE public.crypto;
       public         postgres    false            �            1259    33583    crypto_id_seq    SEQUENCE     �   CREATE SEQUENCE public.crypto_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 $   DROP SEQUENCE public.crypto_id_seq;
       public       postgres    false    204            d           0    0    crypto_id_seq    SEQUENCE OWNED BY     ?   ALTER SEQUENCE public.crypto_id_seq OWNED BY public.crypto.id;
            public       postgres    false    203            �            1259    33616    orders    TABLE     �  CREATE TABLE public.orders (
    id integer NOT NULL,
    user_id integer NOT NULL,
    cryptocurrency_id integer NOT NULL,
    amount double precision NOT NULL,
    price double precision NOT NULL,
    otype character varying(50) NOT NULL,
    created_on timestamp with time zone,
    modified_on timestamp with time zone,
    ostatus character varying(50) NOT NULL,
    market_true boolean NOT NULL
);
    DROP TABLE public.orders;
       public         postgres    false            �            1259    33614    orders_id_seq    SEQUENCE     �   CREATE SEQUENCE public.orders_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 $   DROP SEQUENCE public.orders_id_seq;
       public       postgres    false    210            e           0    0    orders_id_seq    SEQUENCE OWNED BY     ?   ALTER SEQUENCE public.orders_id_seq OWNED BY public.orders.id;
            public       postgres    false    209            �            1259    33600 	   realmoney    TABLE       CREATE TABLE public.realmoney (
    id integer NOT NULL,
    user_id integer NOT NULL,
    currency character varying(255) NOT NULL,
    balance double precision NOT NULL,
    created_on timestamp with time zone,
    modified_on timestamp with time zone
);
    DROP TABLE public.realmoney;
       public         postgres    false            �            1259    33598    realmoney_id_seq    SEQUENCE     �   CREATE SEQUENCE public.realmoney_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 '   DROP SEQUENCE public.realmoney_id_seq;
       public       postgres    false    206            f           0    0    realmoney_id_seq    SEQUENCE OWNED BY     E   ALTER SEQUENCE public.realmoney_id_seq OWNED BY public.realmoney.id;
            public       postgres    false    205            �            1259    33624    trade    TABLE     <  CREATE TABLE public.trade (
    id integer NOT NULL,
    buyer_id integer NOT NULL,
    seller_id integer NOT NULL,
    cryptocurrency_id integer NOT NULL,
    amount double precision NOT NULL,
    price double precision NOT NULL,
    created_on timestamp with time zone,
    modified_on timestamp with time zone
);
    DROP TABLE public.trade;
       public         postgres    false            �            1259    33622    trade_id_seq    SEQUENCE     �   CREATE SEQUENCE public.trade_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 #   DROP SEQUENCE public.trade_id_seq;
       public       postgres    false    212            g           0    0    trade_id_seq    SEQUENCE OWNED BY     =   ALTER SEQUENCE public.trade_id_seq OWNED BY public.trade.id;
            public       postgres    false    211            �            1259    33608    transactions    TABLE     �  CREATE TABLE public.transactions (
    id integer NOT NULL,
    user_id integer NOT NULL,
    wallet_id integer NOT NULL,
    rwallet_id integer NOT NULL,
    cryptocurrency_id integer NOT NULL,
    ttype character varying(50) NOT NULL,
    amount double precision NOT NULL,
    created_on timestamp with time zone,
    modified_on timestamp with time zone,
    payment_method character varying(50) NOT NULL,
    payment_amount double precision NOT NULL,
    payment_status character varying(50) NOT NULL
);
     DROP TABLE public.transactions;
       public         postgres    false            �            1259    33606    transactions_id_seq    SEQUENCE     �   CREATE SEQUENCE public.transactions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 *   DROP SEQUENCE public.transactions_id_seq;
       public       postgres    false    208            h           0    0    transactions_id_seq    SEQUENCE OWNED BY     K   ALTER SEQUENCE public.transactions_id_seq OWNED BY public.transactions.id;
            public       postgres    false    207            �            1259    33567    user_details    TABLE       CREATE TABLE public.user_details (
    id integer NOT NULL,
    user_name character varying(100) NOT NULL,
    email character varying(100) NOT NULL,
    password character varying(100) NOT NULL,
    created_on timestamp with time zone,
    modified_on timestamp with time zone
);
     DROP TABLE public.user_details;
       public         postgres    false            �            1259    33565    user_details_id_seq    SEQUENCE     �   CREATE SEQUENCE public.user_details_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 *   DROP SEQUENCE public.user_details_id_seq;
       public       postgres    false    200            i           0    0    user_details_id_seq    SEQUENCE OWNED BY     K   ALTER SEQUENCE public.user_details_id_seq OWNED BY public.user_details.id;
            public       postgres    false    199            �            1259    24578    users    TABLE     A  CREATE TABLE public.users (
    id integer NOT NULL,
    name character varying(100) NOT NULL,
    email character varying(100) NOT NULL,
    password character varying(100) NOT NULL,
    created_on timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
    modified_on timestamp with time zone DEFAULT CURRENT_TIMESTAMP
);
    DROP TABLE public.users;
       public         postgres    false            �            1259    24576    users_id_seq    SEQUENCE     �   CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 #   DROP SEQUENCE public.users_id_seq;
       public       postgres    false    198            j           0    0    users_id_seq    SEQUENCE OWNED BY     =   ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;
            public       postgres    false    197            �            1259    33577    wallet    TABLE     �   CREATE TABLE public.wallet (
    id integer NOT NULL,
    user_id integer NOT NULL,
    cryptocurrency_id integer NOT NULL,
    balance double precision NOT NULL,
    created_on timestamp with time zone,
    modified_on timestamp with time zone
);
    DROP TABLE public.wallet;
       public         postgres    false            �            1259    33575    wallet_id_seq    SEQUENCE     �   CREATE SEQUENCE public.wallet_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 $   DROP SEQUENCE public.wallet_id_seq;
       public       postgres    false    202            k           0    0    wallet_id_seq    SEQUENCE OWNED BY     ?   ALTER SEQUENCE public.wallet_id_seq OWNED BY public.wallet.id;
            public       postgres    false    201            �
           2604    33588 	   crypto id    DEFAULT     f   ALTER TABLE ONLY public.crypto ALTER COLUMN id SET DEFAULT nextval('public.crypto_id_seq'::regclass);
 8   ALTER TABLE public.crypto ALTER COLUMN id DROP DEFAULT;
       public       postgres    false    204    203    204            �
           2604    33619 	   orders id    DEFAULT     f   ALTER TABLE ONLY public.orders ALTER COLUMN id SET DEFAULT nextval('public.orders_id_seq'::regclass);
 8   ALTER TABLE public.orders ALTER COLUMN id DROP DEFAULT;
       public       postgres    false    209    210    210            �
           2604    33603    realmoney id    DEFAULT     l   ALTER TABLE ONLY public.realmoney ALTER COLUMN id SET DEFAULT nextval('public.realmoney_id_seq'::regclass);
 ;   ALTER TABLE public.realmoney ALTER COLUMN id DROP DEFAULT;
       public       postgres    false    206    205    206            �
           2604    33627    trade id    DEFAULT     d   ALTER TABLE ONLY public.trade ALTER COLUMN id SET DEFAULT nextval('public.trade_id_seq'::regclass);
 7   ALTER TABLE public.trade ALTER COLUMN id DROP DEFAULT;
       public       postgres    false    212    211    212            �
           2604    33611    transactions id    DEFAULT     r   ALTER TABLE ONLY public.transactions ALTER COLUMN id SET DEFAULT nextval('public.transactions_id_seq'::regclass);
 >   ALTER TABLE public.transactions ALTER COLUMN id DROP DEFAULT;
       public       postgres    false    207    208    208            �
           2604    33570    user_details id    DEFAULT     r   ALTER TABLE ONLY public.user_details ALTER COLUMN id SET DEFAULT nextval('public.user_details_id_seq'::regclass);
 >   ALTER TABLE public.user_details ALTER COLUMN id DROP DEFAULT;
       public       postgres    false    200    199    200            �
           2604    24581    users id    DEFAULT     d   ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);
 7   ALTER TABLE public.users ALTER COLUMN id DROP DEFAULT;
       public       postgres    false    198    197    198            �
           2604    33580 	   wallet id    DEFAULT     f   ALTER TABLE ONLY public.wallet ALTER COLUMN id SET DEFAULT nextval('public.wallet_id_seq'::regclass);
 8   ALTER TABLE public.wallet ALTER COLUMN id DROP DEFAULT;
       public       postgres    false    201    202    202            M          0    16394    __diesel_schema_migrations 
   TABLE DATA               E   COPY public.__diesel_schema_migrations (version, run_on) FROM stdin;
    public       postgres    false    196   �Q       U          0    33585    crypto 
   TABLE DATA               S   COPY public.crypto (id, cname, symbol, price, created_on, modified_on) FROM stdin;
    public       postgres    false    204   ?R       [          0    33616    orders 
   TABLE DATA               �   COPY public.orders (id, user_id, cryptocurrency_id, amount, price, otype, created_on, modified_on, ostatus, market_true) FROM stdin;
    public       postgres    false    210   \R       W          0    33600 	   realmoney 
   TABLE DATA               \   COPY public.realmoney (id, user_id, currency, balance, created_on, modified_on) FROM stdin;
    public       postgres    false    206   
S       ]          0    33624    trade 
   TABLE DATA               s   COPY public.trade (id, buyer_id, seller_id, cryptocurrency_id, amount, price, created_on, modified_on) FROM stdin;
    public       postgres    false    212   �S       Y          0    33608    transactions 
   TABLE DATA               �   COPY public.transactions (id, user_id, wallet_id, rwallet_id, cryptocurrency_id, ttype, amount, created_on, modified_on, payment_method, payment_amount, payment_status) FROM stdin;
    public       postgres    false    208   �S       Q          0    33567    user_details 
   TABLE DATA               _   COPY public.user_details (id, user_name, email, password, created_on, modified_on) FROM stdin;
    public       postgres    false    200   wT       O          0    24578    users 
   TABLE DATA               S   COPY public.users (id, name, email, password, created_on, modified_on) FROM stdin;
    public       postgres    false    198   4U       S          0    33577    wallet 
   TABLE DATA               b   COPY public.wallet (id, user_id, cryptocurrency_id, balance, created_on, modified_on) FROM stdin;
    public       postgres    false    202   QU       l           0    0    crypto_id_seq    SEQUENCE SET     <   SELECT pg_catalog.setval('public.crypto_id_seq', 1, false);
            public       postgres    false    203            m           0    0    orders_id_seq    SEQUENCE SET     ;   SELECT pg_catalog.setval('public.orders_id_seq', 5, true);
            public       postgres    false    209            n           0    0    realmoney_id_seq    SEQUENCE SET     >   SELECT pg_catalog.setval('public.realmoney_id_seq', 5, true);
            public       postgres    false    205            o           0    0    trade_id_seq    SEQUENCE SET     ;   SELECT pg_catalog.setval('public.trade_id_seq', 1, false);
            public       postgres    false    211            p           0    0    transactions_id_seq    SEQUENCE SET     A   SELECT pg_catalog.setval('public.transactions_id_seq', 6, true);
            public       postgres    false    207            q           0    0    user_details_id_seq    SEQUENCE SET     A   SELECT pg_catalog.setval('public.user_details_id_seq', 5, true);
            public       postgres    false    199            r           0    0    users_id_seq    SEQUENCE SET     ;   SELECT pg_catalog.setval('public.users_id_seq', 1, false);
            public       postgres    false    197            s           0    0    wallet_id_seq    SEQUENCE SET     ;   SELECT pg_catalog.setval('public.wallet_id_seq', 3, true);
            public       postgres    false    201            �
           2606    16399 :   __diesel_schema_migrations __diesel_schema_migrations_pkey 
   CONSTRAINT     }   ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);
 d   ALTER TABLE ONLY public.__diesel_schema_migrations DROP CONSTRAINT __diesel_schema_migrations_pkey;
       public         postgres    false    196            �
           2606    33595    crypto crypto_cname_key 
   CONSTRAINT     S   ALTER TABLE ONLY public.crypto
    ADD CONSTRAINT crypto_cname_key UNIQUE (cname);
 A   ALTER TABLE ONLY public.crypto DROP CONSTRAINT crypto_cname_key;
       public         postgres    false    204            �
           2606    33593    crypto crypto_pkey 
   CONSTRAINT     P   ALTER TABLE ONLY public.crypto
    ADD CONSTRAINT crypto_pkey PRIMARY KEY (id);
 <   ALTER TABLE ONLY public.crypto DROP CONSTRAINT crypto_pkey;
       public         postgres    false    204            �
           2606    33597    crypto crypto_symbol_key 
   CONSTRAINT     U   ALTER TABLE ONLY public.crypto
    ADD CONSTRAINT crypto_symbol_key UNIQUE (symbol);
 B   ALTER TABLE ONLY public.crypto DROP CONSTRAINT crypto_symbol_key;
       public         postgres    false    204            �
           2606    33621    orders orders_pkey 
   CONSTRAINT     P   ALTER TABLE ONLY public.orders
    ADD CONSTRAINT orders_pkey PRIMARY KEY (id);
 <   ALTER TABLE ONLY public.orders DROP CONSTRAINT orders_pkey;
       public         postgres    false    210            �
           2606    33605    realmoney realmoney_pkey 
   CONSTRAINT     V   ALTER TABLE ONLY public.realmoney
    ADD CONSTRAINT realmoney_pkey PRIMARY KEY (id);
 B   ALTER TABLE ONLY public.realmoney DROP CONSTRAINT realmoney_pkey;
       public         postgres    false    206            �
           2606    33629    trade trade_pkey 
   CONSTRAINT     N   ALTER TABLE ONLY public.trade
    ADD CONSTRAINT trade_pkey PRIMARY KEY (id);
 :   ALTER TABLE ONLY public.trade DROP CONSTRAINT trade_pkey;
       public         postgres    false    212            �
           2606    33613    transactions transactions_pkey 
   CONSTRAINT     \   ALTER TABLE ONLY public.transactions
    ADD CONSTRAINT transactions_pkey PRIMARY KEY (id);
 H   ALTER TABLE ONLY public.transactions DROP CONSTRAINT transactions_pkey;
       public         postgres    false    208            �
           2606    33574 #   user_details user_details_email_key 
   CONSTRAINT     _   ALTER TABLE ONLY public.user_details
    ADD CONSTRAINT user_details_email_key UNIQUE (email);
 M   ALTER TABLE ONLY public.user_details DROP CONSTRAINT user_details_email_key;
       public         postgres    false    200            �
           2606    33572    user_details user_details_pkey 
   CONSTRAINT     \   ALTER TABLE ONLY public.user_details
    ADD CONSTRAINT user_details_pkey PRIMARY KEY (id);
 H   ALTER TABLE ONLY public.user_details DROP CONSTRAINT user_details_pkey;
       public         postgres    false    200            �
           2606    24587    users users_email_key 
   CONSTRAINT     Q   ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (email);
 ?   ALTER TABLE ONLY public.users DROP CONSTRAINT users_email_key;
       public         postgres    false    198            �
           2606    24585    users users_pkey 
   CONSTRAINT     N   ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);
 :   ALTER TABLE ONLY public.users DROP CONSTRAINT users_pkey;
       public         postgres    false    198            �
           2606    33582    wallet wallet_pkey 
   CONSTRAINT     P   ALTER TABLE ONLY public.wallet
    ADD CONSTRAINT wallet_pkey PRIMARY KEY (id);
 <   ALTER TABLE ONLY public.wallet DROP CONSTRAINT wallet_pkey;
       public         postgres    false    202            M   v   x�uϻ�0�:�"��Ey�?G���-�@�|7���\�w��E��Q�!���5�CI}��H���4��As������}.���b^�f'y����2�/���ھ��C?�      U      x������ � �      [   �   x�u�M!��u9�{3�k�P8�+�q5Q����L�!�7}Ba���]��� ��	i�׈��-��{���k�\�3���4S�E�0�ali ~j	�(41Sѱ�ų�a,�*w`��@#��e�����5E�����o�m��'�#Ǧ$�M3dh�k�̃w�=v�S      W   |   x�}ϻ�0Eњ�"}`��'���A���ϑ(�;������N �ڂ��� ��f�� �7b^�WKPL�do�����FW��0�Tؑ-uݵ�)J�UVuLݵv�� װ���kW��\J�`tC�      ]      x������ � �      Y   �   x����
�0Eד�p/�ȣ�oqg����cR�(���@��9��r�a�9�Y�R'��h�Za=�_��2u�}�#"���t�7�=��Q�����==I��Q�1p��
�{r.M�<%G��DKB+�`�˞�&e!�-uh��r�Jm�4��<��X��qE��o��c�x
      Q   �   x���1�0��99;jd;q�tb`b� ,��%�p|RZo��I���2�S�=��Guϣ��������5j��v?��)��
����!4��7k�s�$�E��7DXC\��n��XV,[fY0����j-5LWk'��Zl8�\��b�X\hE����k�'�b��?뎏e      O      x������ � �      S   X   x�u̻�0E�:���z�%8���!�D��ґ"3�ZE��d8�u�t?�{����qp�*d���M�J�cϮW���M�f"z �k&     