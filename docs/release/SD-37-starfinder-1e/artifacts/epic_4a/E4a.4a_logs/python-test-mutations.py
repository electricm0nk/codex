import sys, unittest
sys.path.insert(0, 'scripts/tests')
import test_transcribers_write_the_data_package as T
results = {}
# M1: rules-words is identity
orig = T.tmt.in_the_rules_words
T.tmt.in_the_rules_words = lambda d, v: (d, list(v))
r = unittest.TextTestRunner(stream=open('/dev/null','w')).run(unittest.defaultTestLoader.loadTestsFromTestCase(T.MonsterDescriptionArgumentsSayTheRulesWords))
results['M1 rules-words identity'] = (len(r.failures)+len(r.errors))
T.tmt.in_the_rules_words = orig
# M2: external ref guards not split
orig = T.ttc.split_external_ref_guards
T.ttc.split_external_ref_guards = lambda refs: (list(refs), [])
r = unittest.TextTestRunner(stream=open('/dev/null','w')).run(unittest.defaultTestLoader.loadTestsFromName('test_transcribers_write_the_data_package.CompanionGuardsAreTyped.test_an_external_ref_guard_gates_the_ability_before_it'))
results['M2 ext-ref guards unsplit'] = (len(r.failures)+len(r.errors))
T.ttc.split_external_ref_guards = orig
print(results)
