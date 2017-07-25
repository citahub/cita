#!/bin/bash

delete()
{
PGPASSWORD=postgres psql -h localhost -d postgres -U postgres << EOF
\c $1
delete from transactions;
delete from logs;
delete from nonces;
delete from accounts where identifier in (select identifier from ipay_accounts where role <> 0);
delete from pubkeys where account_id <> 1;
delete from ipay_accounts where role <> 0;
EOF
}

delete node0
delete node1
delete node2
delete node3
