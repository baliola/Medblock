import { usePatientQuery } from "@/services/patients";
import { usePatientStore } from "@/store/patients.store";
// import { useSearchParams } from "next/navigation";
// import { useState } from "react";

const usePatient = () => {
  // const params = useSearchParams();
  // const page = params.get('page') || "1";
  // const limit = params.get('limit') || "10";

  const { setPatients } = usePatientStore()
  
  // const [patientTotalPages, setPatientTotalPages] = useState(0)
  // const [providerTotalCount, setProviderTotalCount] = useState(0)
  
  const { call: getPatientListAdmin, loading: loadingGetPatientList } = usePatientQuery({
    functionName: "get_patient_list_admin",
    refetchOnMount: true,
  });

  const handleGetPatientList = async () => {
    setPatients(undefined)

    await getPatientListAdmin([])
      .then((data) => {
        const result = data?.patients ?? [];
        setPatients(result);
        // setPatientTotalPages(Number(data?.total_pages))
      })
      .catch((error) => {
        setPatients([])
        console.error(error);
      });
  };
  
  return {
    loadingGetPatientList,
    handleGetPatientList,
  }
}

export default usePatient