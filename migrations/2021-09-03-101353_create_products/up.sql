-- Your SQL goes here
CREATE TABLE IF NOT EXISTS public.products
(
    id serial, 
    product_id character varying NOT NULL,
    garment_style character varying NOT NULL,
    garment_color character varying NOT NULL,
    garment_size character varying NOT NULL,
    quantity integer NOT NULL DEFAULT 0,
    created_at TIMESTAMP with time zone NOT NULL DEFAULT now(),
    updated_at TIMESTAMP with time zone DEFAULT now(),
    PRIMARY KEY (id),
    CONSTRAINT unique_product_id UNIQUE (product_id)
);

ALTER TABLE public.products
    OWNER to postgres;
