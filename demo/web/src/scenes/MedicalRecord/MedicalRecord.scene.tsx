import InputTextArea from '@/components/Input/InputFTextArea';
import OptionField from '@/components/Input/OptionField';
import { dummyDoctorTypes } from '@/interface/doctor.interface';
import { NextPageWithLayout } from '@/types';
import { Formik } from 'formik';
import Link from 'next/link';
import React, { useState } from 'react';
import ImageGallery from './component/ImageGallery';
import Modal from '@/components/Modal/Modal';
import InputField from '@/components/Input/InputField';
import { PlusIcon } from '@heroicons/react/20/solid';

const MedicalRecord: NextPageWithLayout = () => {
  const initialValue = {
    name: '',
  };
  const [showModal, setShowModal] = useState(false);
  const [selectedImage, setSelectedImage] = useState('');
  const [query, setQuery] = useState('');
  const handleSubmit = async (values: any) => {
    console.log('values', values);
  };
  const toggleModal = () => {
    setShowModal(!showModal);
  };
  return (
    <>
      <div className="flex flex-col w-full gap-6">
        <div className="flex flex-col gap-1">
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
        <Formik initialValues={initialValue} onSubmit={handleSubmit}>
          {({
            errors,
            handleChange,
            handleSubmit,
            isSubmitting,
            values,
            setFieldValue,
          }) => (
            <div className="flex w-full gap-8">
              <div className="flex flex-col w-1/2 gap-8">
                <div className="flex flex-col gap-4">
                  <div className="flex gap-3">
                    <p className="max-w-[164px] w-full">Med. Record Number</p>
                    <p>: 1234xxxxxx</p>
                  </div>
                  <div className="flex gap-3">
                    <p className="max-w-[164px] w-full">Location</p>
                    <p>: Rumah Sakit Pusat Jakarta Selatan</p>
                  </div>
                  <div className="flex gap-3 items-center">
                    <p className="max-w-[164px] w-full">Date</p>
                    <div className="flex relative max-w-sm gap-2 items-center">
                      :
                      <input
                        type="date"
                        className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-2xl focus:ring-blue-500 focus:border-blue-500 block w-full  p-2 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 "
                        placeholder="Select date"
                      />
                    </div>{' '}
                  </div>
                  <div className="flex gap-3 items-center">
                    <p className="max-w-[164px] w-full">Physician</p>
                    <div className="flex relative max-w-sm gap-2 items-center">
                      :
                      <OptionField
                        options={dummyDoctorTypes}
                        fieldName=""
                        query={query}
                        setQuery={setQuery}
                        setFieldValue={setFieldValue}
                        handleChange={handleChange('doctor')}
                        values={values.name}
                      />
                    </div>{' '}
                  </div>
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
              <div className="flex flex-col w-1/2 gap-20">
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
                    <p className="max-w-[164px] w-full">Blood Presure </p>
                    <InputField
                      fieldName={'blood'}
                      //   label="Blood Presure"
                      handleChange={handleChange('blood')}
                      errors={errors}
                      isPassword={false}
                    />
                  </div>
                  <div className="flex gap-3 items-center">
                    <p className="max-w-[164px] w-full">Oxygen Level </p>
                    <InputField
                      fieldName={'oxygen'}
                      //   label="Blood Presure"
                      handleChange={handleChange('oxygen')}
                      errors={errors}
                      isPassword={false}
                    />
                  </div>
                  <div className="flex gap-3 items-center">
                    <p className="max-w-[164px] w-full">
                      Body Temperature (C){' '}
                    </p>
                    <InputField
                      fieldName={'temperature'}
                      handleChange={handleChange('temperature')}
                      errors={errors}
                      isPassword={false}
                    />
                  </div>
                </div>
                <div className="flex w-full justify-end items-end">
                  <button
                    className="flex  items-center border-[2px] p-2 w-auto outline-hover justify-center align-middle  bg-[#242DA8] transition-all ease-in duration-200 text-white rounded-2xl  border-none text-[14px] font-normal hover:bg-opacity-40"
                    // onClick={() => {
                    //   router.push(`/medical-record/add`);
                    // }}
                  >
                    {/* <img src={} alt="" /> */}
                    <PlusIcon width={16} />
                    Add Medical Record
                  </button>
                </div>
              </div>
            </div>
          )}
        </Formik>
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

export default MedicalRecord;
MedicalRecord.disableLayout = true;
