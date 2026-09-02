import type { CharacterBioDto } from '../boundary/characterBio';
import { formatHeight, type AgeCategory, type Sex } from './characterHubModel';

/**
 * Maps the create form's bio state onto `CharacterBioDto` (v0.8 F-1, scout
 * audit item 2) so it can be persisted with `update_character_bio` right
 * after a `saved` outcome. Formatting only: values are written in the same
 * shapes the sheet's bio panel already reads back. Player name is not part
 * of the DTO yet (backend B-1).
 */
export interface CreationBioFields {
  alignment: string;
  deity: string;
  sex: Sex;
  age: AgeCategory;
  eyes: string;
  hair: string;
  heightInches: number | null;
  weightLb: number | null;
}

export function composeCreationBio(fields: CreationBioFields): CharacterBioDto {
  return {
    alignment: fields.alignment,
    deity: fields.deity,
    sex: fields.sex,
    age: fields.age,
    height: fields.heightInches === null ? '' : formatHeight(fields.heightInches),
    weight: fields.weightLb === null ? '' : `${fields.weightLb} lb`,
    hair: fields.hair,
    eyes: fields.eyes,
  };
}
