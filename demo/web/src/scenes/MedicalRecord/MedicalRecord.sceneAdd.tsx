import InputTextArea from '@/components/Input/InputFTextArea';
import OptionField from '@/components/Input/OptionField';
import { dummyDoctorTypes } from '@/interface/doctor.interface';
import { NextPageWithLayout } from '@/types';
import { Formik } from 'formik';
import Link from 'next/link';
import React, { useEffect, useState } from 'react';
import ImageGallery from './component/ImageGallery';
import Modal from '@/components/Modal/Modal';
import InputField from '@/components/Input/InputField';
import { PlusIcon } from '@heroicons/react/20/solid';
import AuthBtnSubmit from '@/components/AuthButton/AuthBtnSubmit';
import BtnSubmit from '@/components/AuthButton/SubmitBtn';
import { EmrFragment } from 'declarations/emr_registry/emr_registry.did';
import useEmr from '@/hooks/useEmrProvider';
import useCallEMRCanister from '@/hooks/useEmrCanister';
import { useRouter } from 'next/router';
import useEMRPatient from '@/hooks/useEmrPatient';
import { useAuth } from '@/config/agent';
import SplashScreen from '../Splash/SplashScreen';
import { EmrHeader } from 'declarations/provider_registry/provider_registry.did';

export type MedicalRecordType = {
  id: string;
  emrId: string;
  sessions: string;
};

const MedicalRecordAdd: NextPageWithLayout = () => {
  const [showModal, setShowModal] = useState(false);
  const [selectedImage, setSelectedImage] = useState('');
  const [query, setQuery] = useState('');
  const { createEmr, providerName, initialValues } = useEmr();

  const router = useRouter();
  const params = router.query;
  const emrId = params.id;

  const toggleModal = () => {
    setShowModal(!showModal);
  };

  const handleSubmit = async (values: any) => {
    console.log('values frag', providerName);
    const emrFragments: EmrFragment[] = [];

    // Iterate over the initial values object
    for (const [key, value] of Object.entries(initialValues)) {
      // Skip fields that are not included in the initial values

      if (key === 'doctor') continue;

      if (values[key] === undefined) continue;

      // Create EmrFragment object and push it to the array
      emrFragments.push({ key, value: values[key] });
    }
    if (values.doctor && values.doctor.name) {
      emrFragments.push({ key: 'doctor', value: values.doctor.name });
    }

    emrFragments.push({ key: 'location', value: providerName });
    console.log('emr fragments', emrFragments);
    createEmr(emrFragments);
  };

  return (
    <>
      <div className="flex p-6 md:p-4 md:pl-24 md:pr-4 mb-6">
        <div className="flex flex-col w-full gap-6">
          <div className="flex flex-col gap-1 pb-4 border-b">
            <p className="text-xl font-medium text-gray-800 w-auto">
              Medical Record List
            </p>{' '}
            <div className="flex text-sm font-medium  gap-2">
              <Link href="/" className="text-[#06B8EE] underline">
                Patient Management
              </Link>{' '}
              &gt;
              <Link href="/" className="text-[#06B8EE] underline">
                Patient EMR
              </Link>{' '}
              <span> &gt; Add EMR</span>
            </div>{' '}
          </div>
          <Formik initialValues={initialValues} onSubmit={handleSubmit}>
            {({
              errors,
              handleChange,
              handleSubmit,
              values,
              setFieldValue,
            }) => (
              <div className="flex flex-col md:flex-row w-full gap-8">
                <div className="flex flex-col md:w-1/2 gap-8">
                  <div className="flex flex-col gap-4">
                    <p className="w-full font-semibold">
                      Medical Record Information
                    </p>
                    <InputField
                      fieldName={'Medical Record Number'}
                      label="Medical Record Number"
                      errors={errors}
                      isPassword={false}
                      isDisable={true}
                      values={emrId}
                    />
                    <InputField
                      fieldName={'Location'}
                      label="Location"
                      errors={errors}
                      isPassword={false}
                      isDisable={true}
                      values={providerName}
                    />
                    {/* <div className="flex gap-3 items-center">
                      <p className="max-w-[164px] w-full">Date</p>
                      <div className="flex relative max-w-sm gap-2 items-center">
                        :
                        <input
                          type="date"
                          className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-2xl focus:ring-blue-500 focus:border-blue-500 block w-full  p-2 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 "
                          placeholder="Select date"
                        />
                      </div>{' '}
                    </div> */}
                    {/* <div className="flex gap-3 items-center">
                    <p className="max-w-[164px] w-full">Physician</p>
                    <div className="flex relative max-w-sm gap-2 items-center">
                      :
                      <OptionField
                        options={dummyDoctorTypes}
                        fieldName="doctor"
                        query={query}
                        setQuery={setQuery}
                        setFieldValue={setFieldValue}
                        handleChange={handleChange('doctor')}
                        values={values.doctor}
                      />
                    </div>{' '}
                  </div> */}
                  </div>
                  <div className="flex flex-col gap-4">
                    <InputTextArea
                      fieldName="amnanesis"
                      handleChange={handleChange('amnanesis')}
                      label="Amnanesis Result:"
                    />
                    <InputTextArea
                      fieldName="medication"
                      handleChange={handleChange('medication')}
                      label="Medication:"
                    />
                  </div>
                </div>
                <div className="flex flex-col md:w-1/2 gap-8">
                  <div className="flex flex-col gap-4">
                    <p className="max-w-[164px] w-full">Lab Result</p>
                    <ImageGallery
                      setShowModal={setShowModal}
                      setSelectedImage={setSelectedImage}
                      //   toggle={toggleModal}
                    />
                  </div>

                  <div className="flex flex-col gap-4">
                    <p className="max-w-[164px] w-full font-semibold">
                      Early Check up result
                    </p>
                    <div className="flex gap-3 items-center">
                      {/* <p className="max-w-[164px] w-full">Blood Presure </p> */}
                      <InputField
                        fieldName={'blood'}
                        label="Blood Presure"
                        handleChange={handleChange('blood')}
                        errors={errors}
                        isPassword={false}
                      />
                    </div>
                    <div className="flex gap-3 items-center">
                      {/* <p className="max-w-[164px] w-full">Oxygen Level </p> */}
                      <InputField
                        fieldName={'oxygen'}
                        label="Oxygen Level"
                        handleChange={handleChange('oxygen')}
                        errors={errors}
                        isPassword={false}
                      />
                    </div>
                    <div className="flex gap-3 items-center">
                      {/* <p className="max-w-[164px] w-full">
                        Body Temperature (C){' '}
                      </p> */}
                      <InputField
                        fieldName={'temperature'}
                        handleChange={handleChange('temperature')}
                        errors={errors}
                        label=" Body Temperature (C)"
                        isPassword={false}
                      />
                    </div>
                  </div>
                  <div className="flex w-full justify-end items-end">
                    <BtnSubmit
                      title={'Add Medical Record'}
                      onSubmit={handleSubmit}
                      disable={false}
                      color={'#242DA8'}
                    />
                    {/* <button
                    className="flex  items-center border-[2px] p-2 w-auto outline-hover justify-center align-middle  bg-[#242DA8] transition-all ease-in duration-200 text-white rounded-2xl  border-none text-[14px] font-normal hover:bg-opacity-40"
                    // onClick={() => {
                    //   router.push(`/medical-record/add`);
                    // }}
                  >
                    <PlusIcon width={16} />
                    Add Medical Record
                  </button> */}
                  </div>
                </div>
              </div>
            )}
          </Formik>
        </div>
      </div>
      <Modal toggle={toggleModal} isOpen={showModal}>
        <div className="max-w-full max-h-full overflow-auto">
          <img
            className="max-w-full max-h-full rounded-2xl"
            src={selectedImage}
            alt="Full Screen"
          />
        </div>
      </Modal>
    </>
  );
};

export default MedicalRecordAdd;
MedicalRecordAdd.patientLayout = true;
