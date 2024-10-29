import { redirect } from "next/navigation";
import { Tabs } from "@chakra-ui/react";
import EMRPatient from "@/components/dashboard/emr";

interface PageProps {
  params: {
    id: string;
  },
}

export default async function EMRPage({ params }: PageProps) {
  const id = params.id

  if (!id) {
    redirect('/dashboard/patients');
  }

  return (
    <Tabs variant="unstyled">
      <EMRPatient id={id} />
    </Tabs>
  )
}