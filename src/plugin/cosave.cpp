#include "cosave.h"

#include "lib.rs.h"

inline const auto CYCLE_RECORD = _byteswap_ulong('CYCL');

namespace cosave
{

	void initializeCosaves()
	{
		logger::info("Initializing cosave serialization...");
		auto* cosave = SKSE::GetSerializationInterface();
		cosave->SetUniqueID(_byteswap_ulong('SOLS'));
		cosave->SetSaveCallback(cosave::gameSavedHandler);
		// cosave->SetRevertCallback(cosave::revertHandler);
		cosave->SetLoadCallback(cosave::gameLoadedHandler);
	}

	void gameSavedHandler(SKSE::SerializationInterface* cosave)
	{
		if (!cosave->OpenRecord(CYCLE_RECORD, 0))
		{
			logger::error("Unable to open record to write cosave data.");
			return;
		}

		// The format is an ad-hoc bag of bytes that we interpret
		// as we wish. So we serialize to a bag of bytes on the Rust side.

		rust::Vec<uint8_t> buffer = serialize_cycles();
		auto pad                  = (buffer.size() % 16);
		if (pad == 0) pad = 16;
		logger::debug("cycles serialized into a Vec<u8> of size={}; pad={};"sv, buffer.size(), pad);
		cosave->WriteRecordData(buffer.size());
		cosave->WriteRecordData(pad);
		cosave->WriteRecordData(&buffer.data, buffer.size());
		// for (uint8_t byte : buffer) { cosave->WriteRecordData(byte); }
		for (int i = 0; i < pad; i++) { cosave->WriteRecordData(0); }
	}

	void gameLoadedHandler(SKSE::SerializationInterface* cosave)
	{
		std::uint32_t type;
		std::uint32_t version;
		std::uint32_t size;

		while (cosave->GetNextRecordInfo(type, version, size))
		{
			if (type == CYCLE_RECORD)
			{
				size_t bufferSize;
				size_t pad;
				rust::Vec<uint8_t> buffer;
				cosave->ReadRecordData(bufferSize);
				cosave->ReadRecordData(pad);
				logger::debug("found our cosave data; need to read a buffer of size={}; pad={}"sv, bufferSize, pad);

				for (; bufferSize > 0; --bufferSize)
				{
					// this feels staggeringly inefficient, but first I gotta make it work
					uint8_t next;
					cosave->ReadRecordData(next);
					buffer.push_back(next);
				}
				cycle_loaded_from_cosave(buffer);
			}
			else { logger::warn("Unknown record type in cosave; type={}", type); }
		}
	}

	/*
	void revertHandler(SKSE::SerializationInterface* cosave)
	{
		// TODO reset if anything to do ?
	}
	*/
}
