import * as Yup from 'yup';

export interface EMR {
  visit_date: string;
  discharge_date: string;
  visit_time: string;
  discharge_time: string;
  medical_officer: string;
  room: string;

  blood_pressure: string;
  temperature: string;
  heart_rate: string;
  respiration: string;
  o2_saturation: string;

  circuit_reason: string;
  illness_history: string;

  pyhsical_exam: string;
  drug_allergy: string;
  food_allergy: string;
  other_allergy: string;

  additional_exam: string;
  primary_diagnosis: string;
  secondary_diagnosis: string;
  surgery: string;
  procedures_and_therapies: string;

  recipe: string;
  discharge_condition: string;
}

export const emrSchema = Yup.object().shape({
  visit_date: Yup.date()
    .required('Visit date is required')
    .test(
      "visit_date",
      "Date is invalid, visit date must be the same day with or before the discharge date",
      function (value) {
        const { discharge_date } = this.parent;
        return value <= discharge_date;
      }
    ),
  discharge_date: Yup.date()
    .required('Discharge date is required')
    .test(
      "discharge_date",
      "Date is invalid, cannot select date in the future",
      function (value) {
        return value ? value <= new Date() : false;
      }
    ),
  visit_time: Yup.string()
    .required('Visit time is required')
    .test(
      "visit_time",
      "Time is invalid, cannot select time in the future",
      function (value) {
        const { visit_date, discharge_date, discharge_time } = this.parent;

        if (visit_date === discharge_date) {
          return value ? value <=  discharge_time : false;
        }
      }
    ),
  discharge_time: Yup.string()
    .required('Discharge time is required'),
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

  circuit_reason: Yup.string()
    .required('Circuit Reason is required'),
  illness_history: Yup.string(),

  pyhsical_exam: Yup.string(),
  drug_allergy: Yup.string()
    .required('Drug Allergy is required'),
  food_allergy: Yup.string()
    .required('Food Allergy is required'),
  other_allergy: Yup.string(),

  additional_exam: Yup.string(),
  primary_diagnosis: Yup.string()
    .required('Primary Diagnosis is required'),
  secondary_diagnosis: Yup.string(),
  surgery: Yup.string(),
  procedures_and_therapies: Yup.string()
    .required('Procedures and Therapies is required'),

  discharge_condition: Yup.string()
    .required('Dischare Condition is required'),

});