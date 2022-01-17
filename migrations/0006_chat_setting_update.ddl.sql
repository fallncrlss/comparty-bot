ALTER TABLE chat_settings
ADD COLUMN commands_for_admin_only bool not null default false;