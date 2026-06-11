# Tidevault Bug Scenario Catalog (60)

Each scenario has a dedicated integration test under `tests/tests/bug_XXX_*.rs`.

| ID | Scenario | Domain focus |
|----|----------|--------------|
| BUG001 | double capture rejected | payments |
| BUG002 | refund exceeds capture | payments |
| BUG003 | ledger posting imbalance | ledger |
| BUG004 | fx stale quote window | fx |
| BUG005 | settlement partial netting | settlement |
| BUG006 | recon amount mismatch | reconcile |
| BUG007 | fraud velocity tripwire | fraud |
| BUG008 | dispute evidence deadline | disputes |
| BUG009 | compliance soft to hard | compliance |
| BUG010 | webhook dead letter | webhooks |
| BUG011 | pricing tier boundary | pricing |
| BUG012 | limit cooldown window | limits |
| BUG013 | routing cascade exhaustion | routing |
| BUG014 | reporting failed export | reporting |
| BUG015 | auth key rotation race | auth |
| BUG016 | audit hash chain break | audit |
| BUG017 | notify suppression window | notify |
| BUG018 | workflow sla timeout | workflow |
| BUG019 | treasury liquidity shortfall | treasury |
| BUG020 | catalog sunset entitlement | catalog |
| BUG021 | payment void after capture | payments |
| BUG022 | rail reject then retry | rails |
| BUG023 | merchant freeze blocks pay | accounts |
| BUG024 | multi currency batch reject | ledger |
| BUG025 | iban checksum failure | kernel |
| BUG026 | weekend ops policy | policy |
| BUG027 | priority escalation flag | scoring |
| BUG028 | metadata size limit | validation |
| BUG029 | external ref required | retries |
| BUG030 | progressive fee threshold | pricing |
| BUG031 | hhi concentration cap | calc |
| BUG032 | amortization last leg | calc |
| BUG033 | expected shortfall tail | risk |
| BUG034 | feature hash stability | analytics |
| BUG035 | batch risk combined | pipeline |
| BUG036 | idempotency key collision | commands |
| BUG037 | event bus filter | bus |
| BUG038 | store fingerprint change | store |
| BUG039 | capture with compliance flow | engine |
| BUG040 | dispute lifecycle flow | engine |
| BUG041 | fx hedged payout flow | engine |
| BUG042 | demo seed count | engine |
| BUG043 | fee quote positive | pricing |
| BUG044 | state illegal transition | state |
| BUG045 | allocate preserves minor units | money |
| BUG046 | business day skip weekend | clock |
| BUG047 | score band critical | fraud |
| BUG048 | policy blocked currency | policy |
| BUG049 | rules blocking hit | rules |
| BUG050 | pipeline high score needs ext | pipeline |
| BUG051 | settlement cycle ok | settlement |
| BUG052 | onboard payment ok | engine |
| BUG053 | refund path flags | payments |
| BUG054 | recon break open | reconcile |
| BUG055 | webhook backoff seconds | webhooks |
| BUG056 | limit hard breach | limits |
| BUG057 | routing selected commit | routing |
| BUG058 | reporting ready state | reporting |
| BUG059 | auth revocation | auth |
| BUG060 | audit seal export | audit |
