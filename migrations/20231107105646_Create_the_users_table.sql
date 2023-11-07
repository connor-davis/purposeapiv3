CREATE TABLE
    IF NOT EXISTS user_basic_profiles (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4 ()),
        first_name TEXT NOT NULL,
        last_name TEXT NOT NULL,
        id_number TEXT NOT NULL,
        age INT NOT NULL,
        gender TEXT NOT NULL,
        ethnicity TEXT NOT NULL
    );

CREATE TABLE
    IF NOT EXISTS user_business_profiles (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4 ()),
        business_name TEXT NOT NULL,
        business_description TEXT,
        business_type TEXT NOT NULL,
        business_registration_number TEXT,
        business_number_of_employees BIGINT
    );

CREATE TABLE
    IF NOT EXISTS user_early_childhood_development_profiles (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4 ()),
        position_at_early_childhood_development TEXT NOT NULL,
        food_from TEXT NOT NULL DEFAULT 'N/A',
        number_of_children BIGINT NOT NULL DEFAULT 0,
        number_of_dependants BIGINT NOT NULL DEFAULT 0,
        number_of_school_kids BIGINT NOT NULL DEFAULT 0,
        number_of_staff BIGINT NOT NULL DEFAULT 0,
        number_of_classrooms BIGINT NOT NULL DEFAULT 0,
        number_of_toilets BIGINT NOT NULL DEFAULT 0,
        number_of_fridges BIGINT NOT NULL DEFAULT 0,
        number_of_water_tanks BIGINT NOT NULL DEFAULT 0,
        garden_size BIGINT NOT NULL DEFAULT 0,
        has_gardener BOOLEAN NOT NULL DEFAULT false,
        has_garden_in_progress BOOLEAN NOT NULL DEFAULT false,
        has_internet_access BOOLEAN NOT NULL DEFAULT false,
        has_working_lights_and_electricity BOOLEAN NOT NULL DEFAULT false,
        has_running_water BOOLEAN NOT NULL DEFAULT false,
        has_stove_or_oven BOOLEAN NOT NULL DEFAULT false,
        is_growing_crops BOOLEAN NOT NULL DEFAULT false,
        is_first_aid_trained BOOLEAN NOT NULL DEFAULT false,
        is_fire_extinguisher_available BOOLEAN NOT NULL DEFAULT false
    );

CREATE TABLE
    IF NOT EXISTS user_bank_profiles (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4 ()),
        account_number TEXT NOT NULL,
        bank_name TEXT NOT NULL,
        bank_branch_code TEXT NOT NULL
    );

CREATE TABLE
    IF NOT EXISTS user_social_profiles (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4 ()),
        website_url TEXT,
        facebook_page_url TEXT,
        instagram_page_url TEXT,
        youtube_channel_url TEXT
    );

CREATE TABLE
    IF NOT EXISTS user_location_profiles (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4 ()),
        street_address TEXT NOT NULL,
        suburb TEXT,
        ward TEXT,
        city TEXT NOT NULL,
        area_code TEXT NOT NULL,
        province TEXT NOT NULL,
        country TEXT NOT NULL
    );

CREATE TABLE
    IF NOT EXISTS user_profiles (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4 ()),
        user_basic_profile UUID,
        user_business_profile UUID,
        user_early_childhood_development_profile UUID,
        user_bank_profile UUID,
        user_social_profile UUID,
        user_location_profile UUID,
        FOREIGN KEY (user_basic_profile) REFERENCES user_basic_profiles (id) ON UPDATE CASCADE ON DELETE SET NULL,
        FOREIGN KEY (user_business_profile) REFERENCES user_business_profiles (id) ON UPDATE CASCADE ON DELETE SET NULL,
        FOREIGN KEY (user_early_childhood_development_profile) REFERENCES user_early_childhood_development_profiles (id) ON UPDATE CASCADE ON DELETE SET NULL,
        FOREIGN KEY (user_bank_profile) REFERENCES user_bank_profiles (id) ON UPDATE CASCADE ON DELETE SET NULL,
        FOREIGN KEY (user_social_profile) REFERENCES user_social_profiles (id) ON UPDATE CASCADE ON DELETE SET NULL,
        FOREIGN KEY (user_location_profile) REFERENCES user_location_profiles (id) ON UPDATE CASCADE ON DELETE SET NULL
    );

CREATE TABLE
    IF NOT EXISTS users (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4 ()),
        email TEXT NOT NULL,
        password TEXT NOT NULL,
        user_type TEXT NOT NULL DEFAULT 'standard',
        user_group TEXT NOT NULL,
        user_profile UUID,
        FOREIGN KEY (user_profile) REFERENCES user_profiles (id) ON UPDATE CASCADE ON DELETE SET NULL
    );

CREATE INDEX user_email_idx ON users (email);