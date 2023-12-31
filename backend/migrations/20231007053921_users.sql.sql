create table if not exists users(
    username varchar(255) not null unique,
    id serial primary key,
    referral_code varchar(255) not null unique,
    referred_by varchar(255) references users(referral_code),
    invited_users_count int default 0,
    created_at timestamp
    with
        time zone default now(),
        updated_at timestamp
    with
        time zone default now()
);

-- Create a trigger function to automatically update a `updated_at`
create or replace function update_timestamp()
returns trigger as $$
begin
   new.updated_at = now();
   return new;
end;
$$ language 'plpgsql';

-- Drop the trigger if it exists
drop trigger if exists update_users_modtime on users;

-- Add the trigger to the users table
create trigger update_users_modtime
before update on users
for each row
execute function update_timestamp();

--add a function to notify incase of new data changes
create or replace function notify_trigger() returns trigger as $$
declare
begin
    notify user_changes;
    return new;
end;
$$ language plpgsql;

drop trigger if exists update_changes_trigger on users;

--trigger to automatically update notify_trigger
create trigger update_changes_trigger
after insert or update or delete on users
for each row execute function notify_trigger();