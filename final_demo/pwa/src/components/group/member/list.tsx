/* eslint-disable react-hooks/exhaustive-deps */
"use client"

import { patientCanisterId } from "@/config/canisters/patient.canister";
import { PatientActor, usePatientQuery } from "@/services/patients";
import { Flex, Grid, GridItem, Icon, Image, Spinner, Text } from "@chakra-ui/react";
import { ReactElement, useEffect, useState } from "react";
import GroupListBackNavigation from "./back-navigation";
import { EmrListPatientResponse, GetGroupDetailsResponse, GetPatientInfoResponse, GroupDetail } from "@/declarations/patient_registry/patient_registry.did";
import { useParams } from "next/navigation";
import AddMemberModal from "./add";
import { IoIosMale, IoIosFemale } from "react-icons/io";
import { useProfileStore } from "@/store/profile-store";
import LeaveGroupModal from "./leave";
import GrantAccessGroupModal from "./grant";
import RevokeAccessGroupModal from "./revoke";

const MemberGroupList = (): ReactElement => {
  const { group_id } = useParams();
  const { profile } = useProfileStore()

  const [groupDetails, setGroupDetails] = useState<GetGroupDetailsResponse | null | undefined>(undefined)
  const [emrGroupInformation, setEmrGroupInformation] = useState<EmrListPatientResponse | null | undefined>(undefined)

  const { call: getGroupDetails, loading: loadingGetGroupDetails } = usePatientQuery({
    functionName: "get_group_details",
    refetchOnMount: false,
    onSuccess(data) {
      console.log(data)
      const { Ok }: any = data;
      if (Ok) setGroupDetails(Ok);
      else setGroupDetails(null)
    },
    onError(error) {
      setGroupDetails(null)
      console.error(error);
    },
  });

  const handleGetGroupDetails = () => {
    setGroupDetails(undefined)
    getGroupDetails([{ page: 1, limit: 1, group_id: Number(group_id) }] as any)
  }

  const { call: getEmrGroupInformation, loading: loadingGetEmrGroupInformation } = usePatientQuery({
    functionName: "view_group_member_emr_information",
    refetchOnMount: false,
    onSuccess(data) {
      console.log(data)
      const { Ok }: any = data;
      if (Ok) setEmrGroupInformation(Ok);
      else setEmrGroupInformation(null)
    },
    onError(error) {
      setEmrGroupInformation(null)
      console.error(error);
    },
  });

  const handleGetEmrGroupInformation = () => {
    setEmrGroupInformation(undefined)
    getEmrGroupInformation([{ 
      page: 1,
      limit: 1, 
      group_id: Number(group_id),
      member_nik: profile?.nik
    }] as any)
  }

  const isMember = (patientInfo: GetPatientInfoResponse, groupDetails: Array<GroupDetail>): boolean => {
    for (let index = 0; index < groupDetails.length; index++) {
      if (groupDetails[index].nik === patientInfo.nik) return true
    }

    return false
  }
  
  useEffect(() => {
    if (typeof(group_id) === 'string' || profile) {
      getGroupDetails([{ page: 1, limit: 1, group_id: Number(group_id) }] as any)
      getEmrGroupInformation([{ page: 1, limit: 1, group_id: Number(group_id), member_nik: profile?.nik }] as any)
    }
  }, [group_id, profile])

  useEffect(() => {
    console.log(profile)
  }, [profile])

  return (
    <>
      {
        groupDetails && profile
          ? <>
            <GroupListBackNavigation props={{
              name: groupDetails.group_name
            }} />    
            <Flex
              maxHeight={"full"}
              pt={12}
            >
              <Flex
                h={"full"}
                overflowY={"auto"}
                w={"full"}
              >
                <Flex
                  w={"full"}
                  h={"full"}
                  pb={"5.5rem"}
                >
                  <Flex
                    w={"full"}
                    direction={"column"}
                    rowGap={2}
                    pb={4}
                  >
                    <Grid
                      templateColumns="repeat(5, 1fr)"
                      py={2}
                      px={0}
                      columnGap={3}
                      alignItems={"start"}
                    >
                      <GridItem
                        colSpan={1}
                        aspectRatio={1/1}
                        background={"rgb(217, 217, 217)"}
                        display={"block"}
                        rounded={"xl"}
                      />
                      <GridItem 
                        colSpan={4}
                        h={"full"}
                        display={"flex"}
                        flexDirection={"column"}
                        justifyContent={"center"}
                        rowGap={1}
                      >
                        <Text
                          fontWeight={600}
                          fontSize={"lg"}
                          textTransform={"capitalize"}
                        >
                          {groupDetails.leader_name} (Leader) {profile.patient.V1.name === groupDetails.leader_name && '(You)'}
                        </Text>
                      </GridItem>
                    </Grid>
                    {
                      groupDetails.group_details.map((member, index) =>
                        <Grid
                          key={index}
                          templateColumns="repeat(5, 1fr)"
                          py={2}
                          px={0}
                          columnGap={3}
                          alignItems={"start"}
                        >
                          <GridItem
                            colSpan={1}
                            aspectRatio={1/1}
                            background={"rgb(217, 217, 217)"}
                            display={"block"}
                            rounded={"xl"}
                          />
                          <GridItem 
                            colSpan={4}
                            h={"full"}
                            display={"flex"}
                            flexDirection={"column"}
                            justifyContent={"center"}
                            rowGap={1}
                          >
                            <Flex
                              direction={"column"}
                              rowGap={1}
                            >
                              <Text
                                fontWeight={600}
                                fontSize={"lg"}
                                textTransform={"capitalize"}
                              >
                                {member.name} {profile.nik === member.nik && '(You)'}
                              </Text>
                              <Flex
                                justifyContent={"start"}
                                alignItems={"center"}
                                columnGap={2}
                              >
                                {member.gender.toLowerCase() === 'male'
                                  ? <Icon as={IoIosMale} color={"blue.500"} boxSize={4} />
                                  : <Icon as={IoIosFemale} color={"pink.500"} boxSize={4} />
                                }
                                <Text
                                  fontSize={"sm"}
                                  textTransform={"capitalize"}
                                >
                                  {member.age} th
                                </Text>
                                <Text
                                  fontSize={"sm"}
                                  textTransform={"capitalize"}
                                >
                                  ({Object.keys(member.role)[0]})
                                </Text>
                              </Flex>
                            </Flex>
                          </GridItem>
                        </Grid>
                      )
                    }
                  </Flex>
                </Flex>
              </Flex>
            </Flex>
            <Flex
              position={"fixed"}
              left={0}
              bottom={20}
              zIndex={20}
              w={"full"}
              bg={"white"}
              flexDirection={"column"}
              py={5}
              px={5}
            >
              {
                !isMember(profile, groupDetails.group_details)
                  ? <AddMemberModal props={{
                    handleGetGroupDetails
                  }} />
                  : <LeaveGroupModal props={{}}/>
              }
              {/* <LeaveGroupModal props={{}}/> */}
              <GrantAccessGroupModal props={{
                nik: profile.nik
              }} />
              <RevokeAccessGroupModal props={{
                nik: profile.nik
              }} />
            </Flex>
          </>
          : groupDetails === null
            ? <NoGroupDetailsView handleGetGroupDetails={handleGetGroupDetails} />
            : <Flex w={"full"} h={"full"}>
            <Flex
              h={"full"}
              w={"full"}
              margin={"auto"}
              justifyContent={"center"}
            >
              <Spinner size={"md"} colorScheme="primary" color="primary.700" my={"auto"} />
            </Flex>
          </Flex>
      }
    </>
  )
}

const NoGroupDetailsView = ({ handleGetGroupDetails }: { handleGetGroupDetails: () => void }) => {
  return (
    <>
      <Flex
        w={"full"}
        direction={'column'}
        gap={5}
        justify={'space-between'}
        h={'full'}
        marginY={'auto'}
      >
        <Flex 
          flex={1} 
          direction={'column'} 
          align={'center'} 
          justify={'center'} 
          gap={5}
          h={"full"}
          marginY={'auto'}
        >
          <Image 
            src="/assets/female-doctor.png" 
            alt="No Group" 
            w={"50%"}
          />
          <Flex 
            direction={'column'} 
            align={'center'} 
            gap={1}
          >
            <Text 
              fontSize={'lg'} 
              fontWeight={'bold'} 
              color={'neutral.700'}
            >
              Failed to get data
            </Text>
          </Flex>
        </Flex>
      </Flex>
      <AddMemberModal props={{
        handleGetGroupDetails
      }} />
    </>
  )
}

export default function PatienMemberGroupList() {
  return (
    <PatientActor canisterId={patientCanisterId}>
      <Flex 
        flex={1}
        w={"full"}
        direction={"column"}
      >
        <MemberGroupList />
      </Flex>
    </PatientActor>
  )
}