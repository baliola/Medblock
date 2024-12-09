import * as Yup from 'yup';

export interface EMR {
  visit_date: string;
  discharge_date: string;
  medical_officer: string;
  room: string;

  blood_pressure: string;
  temperature: string;
  heart_rate: string;
  respiration: string;
  o2_saturation: string;
  subjective: string;
  diagnosis: string;
  planning: string;
  medication: string;
  recipe: string;
}

export const emrSchema = Yup.object().shape({
  visit_date: Yup.date()
    .required('Visit date is required')
    .test(
      "visit_date",
      "Date is invalid, cannot select a date in the future",
      function (value) {
        return value ? value <= new Date() : false;
      }
    ),
    discharge_date: Yup.date()
    .required('Discharge date is required')
    .test(
      "discharge_date",
      "Date is invalid, discharge date must be the same day with or after the visit date",
      function (value) {
        const { visit_date } = this.parent;
        return value >= visit_date;
      }
    ),
  medical_officer: Yup.string()
    .matches(/^[a-zA-Z\s]*$/, 'Medical officer must be alphabetic')
    .required('Medical officer is required'),
  room: Yup.string()
    .required('Room is required'),
  blood_pressure: Yup.string()
    .matches(/^[0-9/]*$/, 'Blood pressure must be numeric')
    .required('Blood pressure is required'),
  temperature: Yup.string()
    .matches(/^[0-9.]*$/, 'Temperature must be numeric')
    .required('Temperature is required'),
  heart_rate: Yup.string()
    .matches(/^[0-9]*$/, 'Heart rate must be numeric')
    .required('Heart rate is required'),
  respiration: Yup.string()
    .matches(/^[0-9/]*$/, 'Respiration must be numeric')
    .required('Respiration is required'),
  o2_saturation: Yup.string()
    .matches(/^[0-9]*$/, 'Oxygen saturation must be numeric')
    .required('Oxygen saturation is required'),
  subjective: Yup.string()
    .required('Subjective is required'),
  diagnosis: Yup.string()
    .required('Diagnosis is required'),
  planning: Yup.string()
    .required('Planning is required'),
  medication: Yup.string()
    .required('Medication is required'),
  recipe: Yup.string()
    .required('Recipe is required'),
});