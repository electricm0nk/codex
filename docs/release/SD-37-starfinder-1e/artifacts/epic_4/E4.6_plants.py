import sys
n=sys.argv[1]; f="src/sf_adapter.rs"; s=open(f).read()
P={
 "P1_theme_as_pick": ('if package.rule(id).is_some_and(|r| r.pool == "theme") {','if false {'),
 "P2_quantity_dropped": ('Some((_, quantity)) => *quantity += 1,','Some((_, _quantity)) => {}'),
 "P3_bulk_skills_skipped": ('    let penalty = condition.check_penalty();\n    if penalty == 0 {','    let penalty = condition.check_penalty();\n    if penalty != 7 {'),
 "P4_stamina_row_dropped": ('            ("sf.stamina", &c.stamina),\n',''),
 "P5_key_ability_choice_ignored": ('            let ability = parse_ability(&choice.selection_id)','            let _ability = parse_ability(&choice.selection_id)'),
}
old,new=P[n]; assert s.count(old)==1,(n,s.count(old)); s=s.replace(old,new)
if n=="P5_key_ability_choice_ignored":
    s=s.replace("if key_ability_choice.replace(ability).is_some_and(|earlier| earlier != ability) {","if key_ability_choice.replace(_ability).is_some_and(|_| false) && false {",1)
    s=s.replace("    let mut key_ability_choice = None;","    let mut key_ability_choice: Option<Ability> = None;",1)
    # then discard the choice afterwards
    s=s.replace("    let build = SfBuild {\n        chassis: SfChassisBuild {","    key_ability_choice = None;\n    let build = SfBuild {\n        chassis: SfChassisBuild {",1)
open(f,"w").write(s)
