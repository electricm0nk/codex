basic="Athletics Brawling Concentration Conversation Education Evasion FirstAid HumanPerception Language LocalExpert Perception Persuasion Stealth".split()
B={'Rockerboy':[2,6,2,2,2,6,6,6,2,4,2,6,2],'Solo':[2,2,2,2,2,6,6,2,2,2,6,2,2],'Netrunner':[2,2,2,2,6,6,2,2,2,2,2,2,6],
'Tech':[2,2,2,2,6,6,6,2,2,2,2,2,2],'Medtech':[2,2,2,6,6,6,2,6,2,2,2,2,2],'Media':[2,2,2,6,2,6,2,6,2,6,6,6,2],
'Lawman':[2,6,2,6,2,6,2,2,2,2,2,2,2],'Exec':[2,2,2,6,6,6,2,6,2,2,2,6,2],'Fixer':[2,2,2,6,2,6,2,6,4,6,2,4,2],'Nomad':[2,6,2,2,2,6,6,2,2,2,4,2,6]}
R={'Rockerboy':[('Composition',6),('Handgun',6),('Melee Weapon',6),('Personal Grooming',4),('Play Instrument (choose 1)',6),('Streetwise',6),('Wardrobe & Style',4)],
'Solo':[('Autofire (x2)',6),('Handgun',6),('Interrogation',6),('Melee Weapon',6),('Resist Torture/Drugs',6),('Shoulder Arms',6),('Tactics',6)],
'Netrunner':[('Basic Tech',6),('Conceal/Reveal Object',6),('Cryptography',6),('Cybertech',6),('Electronics/Security Tech (x2)',6),('Handgun',6),('Library Search',6)],
'Tech':[('Basic Tech',6),('Cybertech',6),('Electronics/Security Tech (x2)',6),('Land Vehicle Tech',6),('Shoulder Arms',6),('Science (choose 1)',6),('Weaponstech',6)],
'Medtech':[('Basic Tech',6),('Cybertech',4),('Deduction',6),('Paramedic (x2)',6),('Resist Torture/Drugs',4),('Science (choose 1)',6),('Shoulder Arms',6)],
'Media':[('Bribery',6),('Composition',6),('Deduction',6),('Handgun',6),('Library Search',4),('Lip Reading',4),('Photography/Film',4)],
'Lawman':[('Autofire (x2)',6),('Criminology',6),('Deduction',6),('Handgun',6),('Interrogation',6),('Shoulder Arms',6),('Tracking',6)],
'Exec':[('Accounting',6),('Bureaucracy',6),('Business',6),('Deduction',6),('Handgun',6),('Lip Reading',6),('Personal Grooming',4)],
'Fixer':[('Bribery',6),('Business',6),('Forgery',6),('Handgun',6),('Pick Lock',4),('Streetwise',6),('Trading',6)],
'Nomad':[('Animal Handling',6),('Drive Land Vehicle',6),('Handgun',6),('Melee Weapon',6),('Tracking',6),('Trading',6),('Wilderness Survival',6)]}
for r in B:
    cost=sum(B[r])+sum(l*(2 if '(x2)' in n else 1) for n,l in R[r])
    # printed example in text: Streetslang language basic counted; cultural lang free (not included)
    print(r,sum(B[r]),sum(l for n,l in R[r]),cost)
